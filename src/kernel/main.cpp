#include <cstddef>
#include <cstdint>
#include <new>

#include "frame_buffer_config.hpp"
#include "pixel_writer.hpp"

void ClearFrame(const FrameBufferConfig& frame_buffer_config,
                PixelWriter* pixel_writer) {
  for (int x = 0; x < frame_buffer_config.horizontal_resolution; ++x) {
    for (int y = 0; y < frame_buffer_config.vertical_resolution; ++y) {
      struct PixelColor color = {255, 255, 255};
      pixel_writer->Write(x, y, color);
    }
  }
}

extern "C" void KernelMain(const FrameBufferConfig& frame_buffer_config) {
  char pixel_writer_buf[sizeof(RGBReserved8BitPerColorPixelWriter)];
  PixelWriter* pixel_writer;

  switch (frame_buffer_config.pixel_format) {
    case kPixelRGBReserved8BitPerColor:
      pixel_writer = new (pixel_writer_buf)
          RGBReserved8BitPerColorPixelWriter(frame_buffer_config);
      break;
    case kPixelBGRReserved8BitPerColor:
      pixel_writer = new (pixel_writer_buf)
          BGRReserved8BitPerColorPixelWriter(frame_buffer_config);
      break;
  }

  ClearFrame(frame_buffer_config, pixel_writer);

  int w = 300;
  int h = 200;
  for (int x = 0; x < w; ++x) {
    for (int y = 0; y < h; ++y) {
      struct PixelColor color = {
          (uint8_t)(255 * x / w), 180, (uint8_t)(255 * y / h)};
      pixel_writer->Write(100 + x, 100 + y, color);
    }
  }

  while (1) __asm__("hlt");
}
