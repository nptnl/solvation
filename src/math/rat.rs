#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rat {
    n: i32,
    d: i32,
}
impl Rat {
    pub fn new(num: i32, den: i32) -> Rat {
        let (greatest, neg): (i32, bool) = gcf(num, den);
        if neg { Rat { n: -num.abs() / greatest, d: den.abs() / greatest } }
        else { Rat { n: num.abs() / greatest, d: den.abs() / greatest } }
    }
    pub fn newint(num: i32) -> Rat {
        Rat { n: num, d: 1 }
    }
}

fn gcf(inp1: i32, inp2: i32) -> (i32, bool) {
    let mut neg: bool = false;
    let (mut n1, mut n2): (i32, i32) = (inp1, inp2);
    if n1 < 0 { neg = !neg; n1 = -n1 }
    if n2 < 0 { neg = !neg; n2 = -n2 }
    loop {
        if n1 < n2 { n2 %= n1 }
        else { n1 %= n2 }
        if n1 == 0 { return (n2, neg) }
        if n2 == 0 { return (n1, neg) }
    }
}