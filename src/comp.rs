use std::ops;
use crate::preset::{exp, ln};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Comp {
    pub r: f64,
    pub i: f64,
}
impl Comp {
    pub fn new(r: f64, i: f64) -> Self {
        Self { r, i }
    }
    pub fn nre(r: f64) -> Self {
        Self { r, i: 0.0 }
    }
    pub fn nim(i: f64) -> Self {
        Self { r: 0.0, i }
    }
    pub fn square(self) -> Self {
        self * self
    }
    pub fn inv(self) -> Self {
        let divisor: f64 = 1.0 / (self.r*self.r + self.i*self.i);
        Self {
            r: self.r * divisor,
            i: -self.i * divisor
        }
    }
    pub fn mag_square(self) -> f64 {
        self.r * self.r + self.i * self.i
    }
    pub fn mag(self) -> f64 {
        real_sqrt(self.r * self.r + self.i * self.i)
    }
    pub fn pow(self, other: Self) -> Self {
        exp( ln(self) * other )
    }
}
impl ops::Neg for Comp {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            r: -self.r,
            i: -self.i,
        }
    }
}
impl ops::Add<Comp> for Comp {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self { r: self.r + other.r, i: self.i + other.i }
    }
}
impl ops::Sub<Comp> for Comp {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self { r: self.r - other.r, i: self.i - other.i }
    }
}
impl ops::Mul<Comp> for Comp {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r - self.i * other.i,
            i: self.i * other.r + self.r * other.i
        }
    }
}
impl ops::Div<Comp> for Comp {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}
impl ops::AddAssign<Comp> for Comp {
    fn add_assign(&mut self, other: Comp) {
        *self = *self + other
    }
}
impl ops::SubAssign<Comp> for Comp {
    fn sub_assign(&mut self, other: Comp) {
        *self = *self - other
    }
}
impl ops::MulAssign<Comp> for Comp {
    fn mul_assign(&mut self, other: Comp) {
        *self = *self * other
    }
}
impl ops::DivAssign<Comp> for Comp {
    fn div_assign(&mut self, other: Comp) {
        *self = *self / other
    }
}

impl std::str::FromStr for Comp {
    type Err = ();
    fn from_str(slice: &str) -> Result<Comp, Self::Err> {
        let mut chlist = slice.chars();
        let last = chlist.clone().count() - 1;
        if chlist.nth(last).unwrap() == 'i' {
            match slice.rfind('+') {
                Some(v) => Ok( Comp {
                    r: slice[..v].parse::<f64>().unwrap(),
                    i: slice[v+1..last].parse::<f64>().unwrap()
                } ),
                None => match slice.rfind('-') {
                    Some(v) => Ok( Comp {
                        r: slice[..v].parse::<f64>().unwrap(),
                        i: -slice[v+1..last].parse::<f64>().unwrap()
                    } ),
                    None => Ok( Comp {
                        r: 0.0,
                        i: slice[..last].parse::<f64>().unwrap()
                    } ),
                },
            }
        } else {
            match slice.parse::<f64>() {
                Ok(v) => Ok(Comp {r: v, i: 0.0 }),
                Err(_) => Err(()),
            }
        }
    }
}
impl std::fmt::Display for Comp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.i < 0.0 {
            write!(f, "{}-{}i", self.r, -self.i)
        } else if self.i > 0.0 {
            write!(f, "{}+{}i", self.r, self.i)
        } else {
            write!(f, "{}", self.r)
        }
    }
}

pub static ZERO: Comp = Comp { r: 0.0, i: 0.0 };
pub static ONE: Comp = Comp { r: 1.0, i: 0.0 };
pub static II: Comp = Comp { r: 0.0, i: 1.0 };


fn real_sqrt(x: f64) -> f64 {
    let (mut t1, mut t2): (f64, f64) = (2.0, 1.0);
    while (t2 - t1).abs() > 0.0001 {
        t1 = t2;
        t2 -= (t2*t2 - x) / (2.0*t2);
    }
    t2
}