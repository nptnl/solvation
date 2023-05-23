use crate::comp::Comp;
use crate::repl::get_five;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BasicFn {
    Exp,
    Sin,
    Cos,
}


fn raw_exp(x: Comp) -> Comp {
    let mut total: Comp = Comp::nre(0.0);
    let mut running: Comp = Comp::nre(1.0);
    for time in 1..8 {
        total += running;
        running *= x / Comp::nre(time as f64);
    }
    total
}
fn exp_real_rf(r: f64) -> (f64, bool, f64) {
    let e: f64 = 2.7182818284;
    let mut neg: bool = false;
    let mut out: f64 = r;
    if out < 0.0 { out = -out; neg = true }
    let mut extra: f64 = 1.0;
    for time in 0..(out / 1.0) as usize {extra *= e; out -= 1.0 }
    (out, neg, extra)
}
fn exp_imag_rf(i: f64) -> (f64, bool) {
    let π: f64 = 3.1415926535;
    let τ: f64 = 6.283185307;
    let out: f64 = i % τ;
    if i > π { return (out - π, true) }
    else if i < -π { return (out + π, true) }
    (out, false)
}
fn raw_sin(x: Comp) -> Comp {
    let mut total: Comp = Comp::nre(0.0);
    let mut running: Comp = x;
    for time in 1..8 {
        total += running;
        running *= x.square() / Comp::nre((2*time * 2*time+1) as f64)
    }
    total
}
fn raw_cos(x: Comp) -> Comp {
    let mut total: Comp = Comp::nre(0.0);
    let mut running: Comp = Comp::nre(1.0);
    for time in 1..8 {
        total += running;
        running *= x.square() / Comp::nre((2*time * 2*time-1) as f64)
    }
    total
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
pub fn sin(x: Comp) -> Comp { raw_sin(x) }
pub fn cos(x: Comp) -> Comp { raw_cos(x) }

pub static PRE_VAR: [([char; 5], Comp); 3] = [
    (['π', ' ', ' ', ' ', ' '], Comp { r: 3.1415926535, i: 0.0 }),
    (['τ', ' ', ' ', ' ', ' '], Comp { r: 6.283185307, i: 0.0 }),
    (['e', ' ', ' ', ' ', ' '], Comp { r: 2.7182818284, i: 0.0 }),
];