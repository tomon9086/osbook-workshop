#include <cstddef>
#include <cstdint>

#include "frame_buffer_config.hpp"

struct PixelColor {
  uint8_t r, g, b;
};

int WritePixel(const FrameBufferConfig& config, int x, int y,
               const PixelColor& c) {
  const int pixel_position = config.pixels_per_scan_line * y + x;
  uint8_t* p = &config.frame_buffer[4 * pixel_position];

  switch (config.pixel_format) {
    case kPixelRGBReserved8BitPerColor:
      p[0] = c.r;
      p[1] = c.g;
      p[2] = c.b;
      return 0;

    case kPixelBGRReserved8BitPerColor:
      p[0] = c.b;
      p[1] = c.g;
      p[2] = c.r;
      return 0;

    default:
      return -1;
  }
}

void ClearFrame(const FrameBufferConfig& frame_buffer_config) {
  for (int x = 0; x < frame_buffer_config.horizontal_resolution; ++x) {
    for (int y = 0; y < frame_buffer_config.vertical_resolution; ++y) {
      struct PixelColor color = {255, 255, 255};
      WritePixel(frame_buffer_config, x, y, color);
    }
  }
}

extern "C" void KernelMain(const FrameBufferConfig& frame_buffer_config) {
  ClearFrame(frame_buffer_config);

  int w = 300;
  int h = 200;
  for (int x = 0; x < w; ++x) {
    for (int y = 0; y < h; ++y) {
      struct PixelColor color = {
          (uint8_t)(255 * x / w), 180, (uint8_t)(255 * y / h)};
      WritePixel(frame_buffer_config, 100 + x, 100 + y, color);
    }
  }

  while (1) __asm__("hlt");
}
