use crate::comp::{Comp, comp_sqrt};

static ZERO: Comp = Comp { r: 0.0, i: 0.0 };
static ONE: Comp = Comp { r: 1.0, i: 0.0 };
static PI: f64 = 3.1415926535;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BasicFn {
    Exponential,
    Sine,
    Cosine,
    NaturalLog,
    LogBase,
    ArcSine,
    ArcCosine,
}

fn raw_exp(x: Comp) -> Comp {
    let mut total: Comp = ZERO;
    let mut running: Comp = ONE;
    for time in 1..8 {
        total += running;
        running *= x / Comp::nre(time as f64);
    }
    total
}
fn raw_ln(x: Comp) -> Comp {
    let centered: Comp = x - ONE;
    let mut total: Comp = ZERO;
    let mut running: Comp = centered;
    for time in 1..16 {
        total += running / Comp::nre(time as f64);
        running *= -centered;
    }
    total
}

fn exp_real_rf(r: f64) -> (f64, bool, f64) {
    let e: f64 = 2.7182818284;
    let mut neg: bool = false;
    let mut out: f64 = r;
    if out < 0.0 { out = -out; neg = true }
    let mut extra: f64 = 1.0;
    for _ in 0..(out / 1.0) as usize {extra *= e; out -= 1.0 }
    (out, neg, extra)
}
fn exp_imag_rf(i: f64) -> (f64, bool) {
    let mut out: f64 = i;
    let mut realflip: bool = false;
    out %= 2.0*PI;
    println!("{}", out);
    if out > PI { out -= 2.0*PI }
    println!("{}", out);
    if out <= -PI { out += 2.0*PI }
    println!("{}", out);
    if out > 0.5*PI || out < -0.5*PI { out = PI - out; realflip = true; }
    println!("{}", out);
    (out, realflip)
}
fn ln_mag_rf(mag: f64) -> (f64, bool, f64) {
    let e: f64 = 2.7182818284;
    let mut out: f64 = mag;
    let mut extra: f64 = 0.0;
    let mut neg: bool = false;
    if out > 1.0 { out = 1.0 / out; neg = true; }
    while out < 0.6 { out *= e; extra += 1.0; }
    (out, neg, extra)
}
fn ln_ang_rf(unit: Comp) -> (Comp, f64) {
    let (r, i, extra): (f64, f64, f64) =
    if unit.r.abs() > unit.i.abs() { 
        if unit.r < 0.0 { (-unit.r, -unit.i, PI) }
        else { (unit.r, unit.i, 0.0) }
    } else {
        if unit.i < 0.0 { (-unit.i, unit.r, -0.5*PI) }
        else { (unit.i, -unit.r, 0.5*PI) }
    };
    ( Comp { r, i }, extra)
}

pub fn exp(x: Comp) -> Comp {
    let (r, rneg, extra): (f64, bool, f64) = exp_real_rf(x.r);
    let (i, rflip): (f64, bool) = exp_imag_rf(x.i);
    let mut out: Comp = raw_exp(Comp { r, i });
    out *= Comp::nre(extra);
    if rneg { out.r = 1.0 / out.r; }
    if rflip { out.r = -out.r; }
    out
}
fn ixp(x: Comp) -> Comp { exp(Comp::nim(1.0) * x) }
pub fn ln(x: Comp) -> Comp {
    let mag: f64 = x.mag();
    let unit: Comp = x / Comp::nre(mag);
    let (mag_fix, neg, ex_r) = ln_mag_rf(mag);
    let (ang_fix, ex_i) = ln_ang_rf(unit);
    if neg {raw_ln(ang_fix / Comp::nre(mag_fix)) + Comp::new(ex_r, ex_i) }
    else { raw_ln(ang_fix * Comp::nre(mag_fix)) + Comp::new(-ex_r, ex_i) }
}
pub fn log(n: Comp, x: Comp) -> Comp {
    ln(x) / ln(n)
}
pub fn sin(x: Comp) -> Comp {
    let series: Comp = ixp(x);
    Comp::nim(-0.5) * (series - series.inv())
}
pub fn cos(x: Comp) -> Comp {
    let series: Comp = ixp(x);
    Comp::nre(0.5) * (series + series.inv())
}

pub fn asin(x: Comp) -> Comp {
    Comp::nre(0.5*PI) + Comp::nim(-1.0) * ln(x - comp_sqrt(x*x - ONE))
}
pub fn acos(x: Comp) -> Comp {
    Comp::nim(-1.0) * ln(x + comp_sqrt(x*x - ONE))
}

pub(crate) static PRE_VAR: [([char; 5], Comp); 3] = [
    (['π', ' ', ' ', ' ', ' '], Comp { r: 3.1415926535, i: 0.0 }),
    (['τ', ' ', ' ', ' ', ' '], Comp { r: 6.283185307, i: 0.0 }),
    (['e', ' ', ' ', ' ', ' '], Comp { r: 2.7182818284, i: 0.0 }),
];