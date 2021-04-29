#include <Guid/FileInfo.h>
#include <Library/MemoryAllocationLib.h>
#include <Library/UefiBootServicesTableLib.h>
#include <Library/UefiLib.h>
#include <Protocol/LoadedImage.h>
#include <Uefi.h>

struct MemoryMap {
  UINTN bufferSize;
  VOID *buffer;
  UINTN memoryMapSize;
  // EFI_MEMORY_DESCRIPTOR *memoryMap;
  UINTN mapKey;
  UINTN descriptorSize;
  UINT32 descriptorVersion;
};

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
  UINTN num_gop_handles = 0;
  EFI_HANDLE *gop_handles = NULL;
  (gBS->LocateHandleBuffer)(ByProtocol,
                            &gEfiGraphicsOutputProtocolGuid,
                            NULL,
                            &num_gop_handles,
                            &gop_handles);

  (gBS->OpenProtocol)(gop_handles[0],
                      &gEfiGraphicsOutputProtocolGuid,
                      (VOID **)gop,
                      image_handle,
                      NULL,
                      EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);

  FreePool(gop_handles);

  return EFI_SUCCESS;
}

EFI_STATUS OpenRootDir(EFI_HANDLE image_handle, EFI_FILE_PROTOCOL **root) {
  EFI_LOADED_IMAGE_PROTOCOL *loaded_image;
  EFI_SIMPLE_FILE_SYSTEM_PROTOCOL *fs;

  (gBS->OpenProtocol)(image_handle,
                      &gEfiLoadedImageProtocolGuid,
                      (VOID **)&loaded_image,
                      image_handle,
                      NULL,
                      EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);

  (gBS->OpenProtocol)(loaded_image->DeviceHandle,
                      &gEfiSimpleFileSystemProtocolGuid,
                      (VOID **)&fs,
                      image_handle,
                      NULL,
                      EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);

  fs->OpenVolume(fs, root);

  return EFI_SUCCESS;
}

EFI_STATUS CallKernel(EFI_HANDLE image_handle, CHAR16 *path) {
  EFI_FILE_PROTOCOL *root_dir;
  EFI_FILE_PROTOCOL *file;

  OpenRootDir(image_handle, &root_dir);
  root_dir->Open(
      root_dir,
      &file,
      path,
      EFI_FILE_MODE_READ | EFI_FILE_MODE_WRITE | EFI_FILE_MODE_CREATE,
      0);

  UINTN file_info_size = sizeof(EFI_FILE_INFO) + sizeof(CHAR16) * 12;
  UINT8 file_info_buffer[file_info_size];
  file->GetInfo(file, &gEfiFileInfoGuid, &file_info_size, file_info_buffer);

  EFI_FILE_INFO *file_info = (EFI_FILE_INFO *)file_info_buffer;
  UINTN file_size = file_info->FileSize;
  EFI_PHYSICAL_ADDRESS base_addr = 0x100000;
  (gBS->AllocatePages)(
      AllocateAddress, EfiLoaderData, (file_size + 0xfff) / 0x1000, &base_addr);
  file->Read(file, &file_size, (VOID *)base_addr);

  UINT64 entry_addr = *(UINT64 *)(base_addr + 24);

  EFI_GRAPHICS_OUTPUT_PROTOCOL *gop;
  OpenGOP(image_handle, &gop);

  typedef void EntryPointType(UINT64, UINT64);
  EntryPointType *entry_point = (EntryPointType *)entry_addr;
  entry_point(gop->Mode->FrameBufferBase, gop->Mode->FrameBufferSize);

  return EFI_SUCCESS;
}

EFI_STATUS EFIAPI UefiMain(EFI_HANDLE image_handle,
                           EFI_SYSTEM_TABLE *system_table) {
  Print(L"Hello, World!\n");

  CHAR8 memMapBuff[4096 * 4];
  struct MemoryMap memMap = {
      sizeof(memMapBuff),
      memMapBuff,
      0,
      0,
      0,
  };

  GetMemoryMap(&memMap);

  CallKernel(image_handle, L"\\kernel.elf");

  Print(L"Done!\n");
  while (1) continue;
  return EFI_SUCCESS;
}
