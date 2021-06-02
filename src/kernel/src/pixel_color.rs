pub struct PixelColor([u8; 3]);

impl PixelColor {
  pub fn new(r: u8, g: u8, b: u8) -> Self {
    Self([r, g, b])
  }
  pub fn to_array(&self) -> [u8; 3] {
    self.0
  }
  pub fn r(&self) -> u8 {
    self.to_array()[0]
  }
  pub fn g(&self) -> u8 {
    self.to_array()[1]
  }
  pub fn b(&self) -> u8 {
    self.to_array()[2]
  }
}
