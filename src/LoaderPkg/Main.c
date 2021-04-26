#include <Library/UefiBootServicesTableLib.h>
#include <Library/UefiLib.h>
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

  // メモリ領域の大きさ 設定と実際の値
  Print(L"%08lx\n", memMap.bufferSize);
  Print(L"%08lx\n", memMap.memoryMapSize);
  // メモリ領域の種別
  Print(L"%08lx\n", ((EFI_MEMORY_DESCRIPTOR *)memMap.buffer)->Type);
  // メモリ領域のページ (4KiB単位) 数
  Print(L"%08lx\n", ((EFI_MEMORY_DESCRIPTOR *)memMap.buffer)->NumberOfPages);

  while (1) continue;
  return EFI_SUCCESS;
}
