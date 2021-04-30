#include <Guid/FileInfo.h>
#include <Library/MemoryAllocationLib.h>
#include <Library/PrintLib.h>
#include <Library/UefiBootServicesTableLib.h>
#include <Library/UefiLib.h>
#include <Protocol/LoadedImage.h>
#include <Uefi.h>

#include "frame_buffer_config.hpp"

struct MemoryMap {
  UINTN bufferSize;
  VOID *buffer;
  UINTN memoryMapSize;
  // EFI_MEMORY_DESCRIPTOR *memoryMap;
  UINTN mapKey;
  UINTN descriptorSize;
  UINT32 descriptorVersion;
};

void Halt(void) {
  while (1) __asm__("hlt");
}

EFI_STATUS GetMemoryMap(struct MemoryMap *map) {
  if (map->buffer == NULL) {
    return EFI_BUFFER_TOO_SMALL;
  }

  map->memoryMapSize = map->bufferSize;
  return (gBS->GetMemoryMap)(&map->memoryMapSize,
                             (EFI_MEMORY_DESCRIPTOR *)map->buffer,
                             &map->mapKey,
                             &map->descriptorSize,
                             &map->descriptorVersion);
}

EFI_STATUS OpenGOP(EFI_HANDLE image_handle,
                   EFI_GRAPHICS_OUTPUT_PROTOCOL **gop) {
  EFI_STATUS status;
  UINTN num_gop_handles = 0;
  EFI_HANDLE *gop_handles = NULL;
  status = (gBS->LocateHandleBuffer)(ByProtocol,
                                     &gEfiGraphicsOutputProtocolGuid,
                                     NULL,
                                     &num_gop_handles,
                                     &gop_handles);
  if (EFI_ERROR(status)) {
    return status;
  }

  status = (gBS->OpenProtocol)(gop_handles[0],
                               &gEfiGraphicsOutputProtocolGuid,
                               (VOID **)gop,
                               image_handle,
                               NULL,
                               EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);
  if (EFI_ERROR(status)) {
    return status;
  }

  FreePool(gop_handles);
  return EFI_SUCCESS;
}

EFI_STATUS OpenRootDir(EFI_HANDLE image_handle, EFI_FILE_PROTOCOL **root) {
  EFI_STATUS status;
  EFI_LOADED_IMAGE_PROTOCOL *loaded_image;
  EFI_SIMPLE_FILE_SYSTEM_PROTOCOL *fs;

  status = (gBS->OpenProtocol)(image_handle,
                               &gEfiLoadedImageProtocolGuid,
                               (VOID **)&loaded_image,
                               image_handle,
                               NULL,
                               EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);
  if (EFI_ERROR(status)) {
    return status;
  }

  status = (gBS->OpenProtocol)(loaded_image->DeviceHandle,
                               &gEfiSimpleFileSystemProtocolGuid,
                               (VOID **)&fs,
                               image_handle,
                               NULL,
                               EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);
  if (EFI_ERROR(status)) {
    return status;
  }

  return fs->OpenVolume(fs, root);
}

EFI_STATUS CallKernel(EFI_HANDLE image_handle, CHAR16 *path) {
  EFI_STATUS status;
  EFI_FILE_PROTOCOL *root_dir;
  EFI_FILE_PROTOCOL *file;

  status = OpenRootDir(image_handle, &root_dir);
  if (EFI_ERROR(status)) {
    return status;
  }
  status = root_dir->Open(
      root_dir,
      &file,
      path,
      EFI_FILE_MODE_READ | EFI_FILE_MODE_WRITE | EFI_FILE_MODE_CREATE,
      0);
  if (EFI_ERROR(status)) {
    return status;
  }

  UINTN file_info_size = sizeof(EFI_FILE_INFO) + sizeof(CHAR16) * 12;
  UINT8 file_info_buffer[file_info_size];
  status =
      file->GetInfo(file, &gEfiFileInfoGuid, &file_info_size, file_info_buffer);
  if (EFI_ERROR(status)) {
    return status;
  }

  EFI_FILE_INFO *file_info = (EFI_FILE_INFO *)file_info_buffer;
  UINTN file_size = file_info->FileSize;
  EFI_PHYSICAL_ADDRESS base_addr = 0x100000;
  status = (gBS->AllocatePages)(
      AllocateAddress, EfiLoaderData, (file_size + 0xfff) / 0x1000, &base_addr);
  if (EFI_ERROR(status)) {
    return status;
  }
  status = file->Read(file, &file_size, (VOID *)base_addr);
  if (EFI_ERROR(status)) {
    return status;
  }

  UINT64 entry_addr = *(UINT64 *)(base_addr + 24);

  EFI_GRAPHICS_OUTPUT_PROTOCOL *gop;
  status = OpenGOP(image_handle, &gop);
  if (EFI_ERROR(status)) {
    return status;
  }

  struct FrameBufferConfig config = {(UINT8 *)gop->Mode->FrameBufferBase,
                                     gop->Mode->Info->PixelsPerScanLine,
                                     gop->Mode->Info->HorizontalResolution,
                                     gop->Mode->Info->VerticalResolution,
                                     0};
  switch (gop->Mode->Info->PixelFormat) {
    case PixelRedGreenBlueReserved8BitPerColor:
      config.pixel_format = kPixelRGBReserved8BitPerColor;
      break;
    case PixelBlueGreenRedReserved8BitPerColor:
      config.pixel_format = kPixelBGRReserved8BitPerColor;
      break;
    default:
      return EFI_UNSUPPORTED;
  }
  typedef void EntryPointType(const struct FrameBufferConfig *);
  EntryPointType *entry_point = (EntryPointType *)entry_addr;
  entry_point(&config);

  return EFI_SUCCESS;
}

EFI_STATUS EFIAPI UefiMain(EFI_HANDLE image_handle,
                           EFI_SYSTEM_TABLE *system_table) {
  EFI_STATUS status;
  CHAR8 memMapBuff[4096 * 4];
  struct MemoryMap memMap = {
      sizeof(memMapBuff),
      memMapBuff,
      0,
      0,
      0,
  };

  status = GetMemoryMap(&memMap);
  if (EFI_ERROR(status)) {
    Print(L"GetMemoryMap - Error: %r\n", status);
    Halt();
  }

  status = CallKernel(image_handle, L"\\kernel.elf");
  if (EFI_ERROR(status)) {
    Print(L"CallKernel - Error: %r\n", status);
    Halt();
  }

  Halt();
  return EFI_SUCCESS;
}
