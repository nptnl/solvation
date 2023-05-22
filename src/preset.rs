use crate::comp::Comp;

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

pub fn exp(x: Comp) -> Comp { raw_exp(x) }
pub fn sin(x: Comp) -> Comp { raw_sin(x) }
pub fn cos(x: Comp) -> Comp { raw_cos(x) }