#include <cstdint>

void operator delete(void* obj) {}

struct PixelColor {
  uint8_t r, g, b;
};

class PixelWriter {
 private:
  const FrameBufferConfig& config_;

 protected:
  uint8_t* PixelAt(int x, int y) {
    return config_.frame_buffer + 4 * (config_.pixels_per_scan_line * y + x);
  }

 public:
  PixelWriter(const FrameBufferConfig& config) : config_(config){};
  virtual ~PixelWriter() = default;
  virtual void Write(int x, int y, const PixelColor& c) = 0;
};

class RGBReserved8BitPerColorPixelWriter : public PixelWriter {
 public:
  RGBReserved8BitPerColorPixelWriter(const FrameBufferConfig& config)
      : PixelWriter(config){};
  virtual void Write(int x, int y, const PixelColor& c) override {
    uint8_t* p = PixelAt(x, y);
    p[0] = c.r;
    p[1] = c.g;
    p[2] = c.b;
  }
};

class BGRReserved8BitPerColorPixelWriter : public PixelWriter {
 public:
  BGRReserved8BitPerColorPixelWriter(const FrameBufferConfig& config)
      : PixelWriter(config){};
  virtual void Write(int x, int y, const PixelColor& c) override {
    uint8_t* p = PixelAt(x, y);
    p[0] = c.b;
    p[1] = c.g;
    p[2] = c.r;
  }
};
