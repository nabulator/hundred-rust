// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SaturatingU16 {
    u: u16
}

impl SaturatingU16 {
    pub fn new(i: u16) -> Self {
        SaturatingU16 {
            u: i
        }
    }
    
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 {u: value.into()}
    }
}
impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 {u: value as u16}
    }
}
impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16 {u: value.clone()} 
    }
}
impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 {u: value.clone() as u16} 
    }
}

use std::ops::Add;
impl Add<SaturatingU16> for SaturatingU16 {
    type Output = u16;
    fn add(self, rhs: SaturatingU16) -> u16 {
        self.u.saturating_add(rhs.u)
    }
}
impl Add<u16> for SaturatingU16 {
    type Output = u16;
    fn add(self, rhs: u16) -> u16 {
        self.u.saturating_add(rhs)
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &u16) -> SaturatingU16 {
        SaturatingU16 {
            u: self.u.saturating_add(rhs.clone())
        } 
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &SaturatingU16) -> SaturatingU16 {
        SaturatingU16 {
            u: self.u.saturating_add(rhs.u)
        } 
    }
}

// impl PartialEq<u16> for SaturatingU16 {
//     fn eq(&self, other: &u16) -> bool {
//         &self.u == other
//     }
// }
impl PartialEq<SaturatingU16> for u16 {
    fn eq(&self, other: &SaturatingU16) -> bool {
        self == &other.u
    }
}
