use crate::comp::Comp;
use crate::repl::{Bat, BinOp, Type};

static ZERO: Comp = Comp { r: 0.0, i: 0.0 };
static ONE: Comp = Comp { r: 1.0, i: 0.0 };
static PI: f64 = 3.1415926535;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BasicFn {

    Exponential,
    NaturalLog,
    LogBase,

    Sine,
    Cosine,
    Tangent,
    Cotangent,
    Secant,
    Cosecant,

    ArcSine,
    ArcCosine,
    ArcTangent,
    ArcCotangent,
    ArcSecant,
    ArcCosecant,

    HypSine,
    HypCosine,
    HypTangent,
    HypCotangent,
    HypSecant,
    HypCosecant,
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
    if out > PI { out -= 2.0*PI }
    else if out <= -PI { out += 2.0*PI }
    if out > 0.5*PI { out = PI - out; realflip = true; }
    else if out < -0.5*PI { out = -PI - out; realflip = true; }
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
pub fn ixp(x: Comp) -> Comp { exp(Comp::nim(1.0) * x) }
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
    
pub(crate) static PRE_VAR: [([char; 5], Comp); 3] = [
    (['π', ' ', ' ', ' ', ' '], Comp { r: 3.1415926535, i: 0.0 }),
    (['2', 'π', ' ', ' ', ' '], Comp { r: 6.283185307, i: 0.0 }),
    (['e', ' ', ' ', ' ', ' '], Comp { r: 2.7182818284, i: 0.0 }),
];


static DX: Comp = Comp { r: 0.00000000001, i: 0.0 };

pub(crate) static LIMIT_DVT: [Bat; 15] = [
    Bat::Begin(1),
    Bat::Inp(1), Bat::Begin(2), Bat::Inp(2), Bat::Rel(BinOp::Add), Bat::Val(Type::C(DX)), Bat::End(2),
    Bat::Rel(BinOp::Sub),
    Bat::Inp(1), Bat::Begin(2), Bat::Inp(2), Bat::End(2),
    Bat::End(1),
    Bat::Rel(BinOp::Div), Bat::Val(Type::C(DX)),
];