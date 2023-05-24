use crate::comp;
use crate::comp::Comp;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BasicFn {
    Exp,
    Sin,
    Cos,
    Ln,
}

fn raw_exp(x: Comp) -> Comp {
    let mut total: Comp = comp::ZERO;
    let mut running: Comp = comp::ONE;
    for time in 1..8 {
        total += running;
        running *= x / Comp::nre(time as f64);
    }
    total
}
fn raw_ln(x: Comp) -> Comp {
    let centered: Comp = x - comp::ONE;
    let mut total: Comp = comp::ZERO;
    let mut running: Comp = centered;
    for time in 1..16 {
        total += running / Comp::nre(time as f64);
        running *= -centered;
    }
    total
}
fn raw_sin(x: Comp) -> Comp {
    let mut total: Comp = comp::ZERO;
    let mut running: Comp = x;
    for time in 1..8 {
        println!("{total}");
        total += running;
        running *= -x*x * Comp::nre(1.0 / (2*time * (2*time+1)) as f64);
    }
    total
}
fn raw_cos(x: Comp) -> Comp {
    let mut total: Comp = comp::ZERO;
    let mut running: Comp = comp::ONE;
    for time in 1..8 {
        total += running;
        running *= -x*x * Comp::nre(1.0 / (2*time * (2*time-1)) as f64);
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
    let pi: f64 = 3.1415926535;
    let tau: f64 = 6.283185307;
    let out: f64 = i % tau;
    if i > pi { return (out - pi, true) }
    else if i < -pi { return (out + pi, true) }
    (out, false)
}
fn anglefix(r: f64) -> (f64, (bool, bool, bool, bool)) {
    let pi: f64 = 3.1415926535;
    let mut ang: f64 = r;
    let mut transform: (bool,bool, bool, bool) = (false, false, false, false);
    if ang < 0.0 { transform.0 = true; ang = -ang }
    ang %= 2.0*pi;
    if ang >= pi { transform.1 = true; ang = 2.0*pi - ang }
    if ang >= 0.5*pi { transform.2 = true; ang = pi - ang }
    if ang >= 0.25*pi { transform.3 = true; ang = 0.5*pi - ang }
    (ang, transform)
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
    let pi: f64 = 3.1415926535;
    let (r, i, extra): (f64, f64, f64) =
    if unit.r.abs() > unit.i.abs() { 
        if unit.r < 0.0 { (-unit.r, -unit.i, pi) }
        else { (unit.r, unit.i, 0.0) }
    } else {
        if unit.i < 0.0 { (-unit.i, unit.r, -0.5*pi) }
        else { (unit.i, -unit.r, 0.5*pi) }
    };
    ( Comp { r, i }, extra)
}

pub fn exp(x: Comp) -> Comp {
    let (r, rneg, extra): (f64, bool, f64) = exp_real_rf(x.r);
    let (i, ineg): (f64, bool) = exp_imag_rf(x.i);
    let mut out: Comp = raw_exp(Comp { r, i });
    out *= Comp::nre(extra);
    if rneg { out = out.inv(); }
    if ineg { out = -out; }
    out
}
pub fn ln(x: Comp) -> Comp {
    let mag: f64 = x.mag();
    let unit: Comp = x / Comp::nre(mag);
    let (mag_fix, neg, ex_r) = ln_mag_rf(mag);
    let (ang_fix, ex_i) = ln_ang_rf(unit);
    if neg { -raw_ln(Comp::nre(mag_fix)) + raw_ln(ang_fix) + Comp::new(ex_r, ex_i) }
    else { raw_ln(Comp::nre(mag_fix)) + raw_ln(ang_fix) + Comp::new(-ex_r, ex_i) }
}
pub fn sin(x: Comp) -> Comp {
    let (r, fix): (f64, (bool, bool, bool, bool)) = anglefix(x.r);
    let mut out: Comp =
    if fix.3 { raw_cos(Comp { r, i: x.i } ) } else { raw_sin(Comp { r, i: x.i } ) };
    if fix.1 { out = -out }
    out
}
pub fn cos(x: Comp) -> Comp {
    let (r, fix): (f64, (bool, bool, bool, bool)) = anglefix(x.r);
    let mut out: Comp =
    if fix.3 { raw_sin(Comp { r, i: x.i } ) } else { raw_sin(Comp { r, i: x.i } ) };
    if fix.2 { out = -out }
    out
}
 
pub static PRE_VAR: [([char; 5], Comp); 3] = [
    (['π', ' ', ' ', ' ', ' '], Comp { r: 3.1415926535, i: 0.0 }),
    (['τ', ' ', ' ', ' ', ' '], Comp { r: 6.283185307, i: 0.0 }),
    (['e', ' ', ' ', ' ', ' '], Comp { r: 2.7182818284, i: 0.0 }),
];