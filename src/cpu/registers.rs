use std::fmt;

pub use self::flags::{Flags, ZERO, ADD_SUBTRACT, HALF_CARRY, CARRY};

#[allow(dead_code)]
mod flags {
  use std::fmt;

  bitflags!(
    flags Flags: u8 {
      const ZERO = 0x80,
      const ADD_SUBTRACT = 0x40,
      const HALF_CARRY = 0x20,
      const CARRY = 0x10
    }
  );

  impl Flags {
    #[inline]
    pub fn test(&self, test: bool) -> Flags { if test { *self } else { Flags::empty() } }
  }

  impl fmt::Binary for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      fmt::Binary::fmt(&(self.bits >> 4), f)
    }
  }

  impl fmt::LowerHex for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      fmt::LowerHex::fmt(&self.bits, f)
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub enum Reg8 {
  A, B, C, D, E, H, L
}

#[derive(Clone, Copy, Debug)]
pub enum Reg16 {
  AF, BC, DE, HL, SP
}

#[derive(Clone, Copy, Debug)]
pub struct Registers {
  pub pc: u16,
  pub sp: u16,
  pub a: u8,
  pub f: Flags,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      pc: 0,
      sp: 0,
      a: 0,
      f: Flags::empty(),
      b: 0,
      c: 0,
      d: 0,
      e: 0,
      h: 0,
      l: 0
    }
  }

  pub fn read16(&self, reg: Reg16) -> u16 {
    use self::Reg16::*;
    match reg {
      AF => ((self.a as u16) << 8) | (self.f.bits() as u16),
      BC => ((self.b as u16) << 8) | (self.c as u16),
      DE => ((self.d as u16) << 8) | (self.e as u16),
      HL => ((self.h as u16) << 8) | (self.l as u16),
      SP => self.sp,
    }
  }

  pub fn write16(&mut self, reg: Reg16, value: u16) {
    use self::Reg16::*;
    match reg {
      AF => { self.a = (value >> 8) as u8; self.f = Flags::from_bits_truncate(value as u8) },
      BC => { self.b = (value >> 8) as u8; self.c = value as u8 },
      DE => { self.d = (value >> 8) as u8; self.e = value as u8 },
      HL => { self.h = (value >> 8) as u8; self.l = value as u8 },
      SP => self.sp = value
    }
  }
}

impl fmt::Display for Registers {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "PC:{:04x} SP:{:04x} \
               A:{:02x} F:{:04b} B:{:02x} C:{:02x} \
               D:{:02x} E:{:02x} H:{:02x} L:{:02x}",
               self.pc, self.sp,
               self.a, self.f, self.b, self.c,
               self.d, self.e, self.h, self.l)
  }
}
