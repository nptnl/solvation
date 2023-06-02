use crate::math::prim::{exp, ixp, ln};
use crate::math::comp::{comp_sqrt, Comp};

static ONE: Comp = Comp { r: 1.0, i: 0.0 };
static I: Comp = Comp { r: 0.0, i: 1.0 };
static PI: f64 = 3.1415926535;

pub fn sin(x: Comp) -> Comp {
    let series: Comp = ixp(x);
    Comp::nim(-0.5) * (series - series.inv())
}
pub fn cos(x: Comp) -> Comp {
    let series: Comp = ixp(x);
    Comp::nre(0.5) * (series + series.inv())
}
pub fn tan(x: Comp) -> Comp {
    let series: Comp = ixp(x);
    Comp::nim(-1.0) * (series - series.inv()) / (series + series.inv())
}
pub fn cot(x: Comp) -> Comp {
    let series: Comp = ixp(x);
    Comp::nim(1.0) * (series + series.inv()) / (series - series.inv())
}
pub fn sec(x: Comp) -> Comp {
    let series: Comp = ixp(x);
    Comp::nre(2.0)  / (series + series.inv())
}
pub fn csc(x: Comp) -> Comp {
    let series: Comp = ixp(x);
    Comp::nim(2.0) / (series - series.inv())
}


pub fn asin(x: Comp) -> Comp {
    Comp::nre(0.5*PI) + Comp::nim(-1.0) * ln(x - comp_sqrt(x*x - ONE))
}
pub fn acos(x: Comp) -> Comp {
    Comp::nim(-1.0) * ln(x + comp_sqrt(x*x - ONE))
}
pub fn atan(x: Comp) -> Comp {
    Comp::nre(0.5*PI) + Comp::nim(-0.5) * ln((I*x + ONE) / (I*x - ONE))
}
pub fn acot(x: Comp) -> Comp {
    Comp::nim(-0.5) * ln((x + I) / (x - I))
}
pub fn acsc(x: Comp) -> Comp { asin(x.inv()) }
pub fn asec(x: Comp) -> Comp { acos(x.inv()) }

pub fn sinh(x: Comp) -> Comp {
    let series: Comp = exp(x);
    Comp::nre(0.5) * (series - series.inv())
}
pub fn cosh(x: Comp) -> Comp {
    let series: Comp = exp(x);
    Comp::nre(0.5) * (series + series.inv())
}
pub fn tanh(x: Comp) -> Comp {
    let series: Comp = exp(x);
    (series - series.inv()) / (series + series.inv())
}
pub fn coth(x: Comp) -> Comp {
    let series: Comp = exp(x);
    (series + series.inv()) / (series -  series.inv())
}
pub fn sech(x: Comp) -> Comp {
    let series: Comp = exp(x);
    Comp::nre(2.0) / (series + series.inv())
}
pub fn csch(x: Comp) -> Comp {
    let series: Comp = exp(x);
    Comp::nre(2.0) / (series - series.inv())
}

pub fn asinh(x: Comp) -> Comp {
    ln(x - comp_sqrt(x*x + ONE))
}
pub fn acosh(x: Comp) -> Comp {
    ln(x - comp_sqrt(x*x - ONE))
}
pub fn atanh(x: Comp) -> Comp {
    Comp::nim(0.5*PI) + Comp::nre(0.5) * ln((x + ONE) / (x - ONE))
}
pub fn acoth(x: Comp) -> Comp {
    Comp::nre(0.5) * ln((x + ONE) / (x - ONE))
}
pub fn asech(x: Comp) -> Comp {
    acosh(x.inv())
}
pub fn acsch(x: Comp) -> Comp {
    asinh(x.inv())
}