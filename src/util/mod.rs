#![allow(non_snake_case)]

pub mod datastore;
pub mod rdf;

/* Common Definitions */

pub type Triple = [String; 3];

#[derive(Clone)]
struct Nibble(u8);
impl Nibble {
  fn new(val: u8) -> Result<Self, ()> {
    if val > 8 {
      return Err(())
    }
    Ok(Nibble(val))
  }
  fn get(&self, bit_pos: u8) -> Result<bool, ()> {
    match bit_pos {
      0 => Ok((**self & 0b1000) != 0),
      1 => Ok((**self & 0b0100) != 0),
      2 => Ok((**self & 0b0010) != 0),
      3 => Ok((**self & 0b0001) != 0),
      _ => Err(())
    }
  }
  fn set(&mut self, bit_pos: u8, val: bool) -> Result<(), ()> {
    if self.get(bit_pos) != Ok(val) {
      **self += match bit_pos {
        0 => 8,
        1 => 4,
        2 => 2,
        3 => 1,
        _ => return Err(()),
      };
    }
    Ok(())
  }
}
impl std::ops::Deref for Nibble {
  type Target = u8;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for Nibble {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}