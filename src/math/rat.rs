use std::{ops, cmp};

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
    pub fn reduce(&mut self) {
        let (greatest, neg): (i32, bool) = gcf(self.n, self.d);
        *self = 
        if neg { Rat { n: -self.n.abs() / greatest, d: self.d.abs() / greatest } }
        else { Rat { n: self.n.abs() / greatest, d: self.d.abs() / greatest } }
    }
    pub fn recip(&self) -> Self {
        Self::new(self.d, self.n)
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

impl PartialOrd for Rat {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let res: Rat = *self - *other;
        res.n.partial_cmp(&0)
    }
}

impl ops::Add<Rat> for Rat {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new( self.n * other.d + other.n * self.d, self.d * other.d )
    }
}
impl ops::Sub<Rat> for Rat {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new( self.n * other.d - other.n * self.d, self.d * other.d )
    }
}
impl ops::Mul<Rat> for Rat {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.n * other.n, self.d * other.d)
    }
}
impl ops::Div<Rat> for Rat {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new(self.n * other.d, self.d * other.n)
    }
}

impl std::str::FromStr for Rat {
    type Err = ();
    fn from_str(slice: &str) -> Result<Self, Self::Err> {
        if slice.len() < 3 { return Err(()) }
        match slice.rfind('/') {
            Some(v) => Ok( Self::new(
                slice[..v].parse::<i32>().unwrap(),
                slice[v+1..].parse::<i32>().unwrap()
            ) ),
            None => Err(()) // Ok( Self::newint(slice.parse::<i32>().unwrap()) )
        }
    }
}
impl std::fmt::Display for Rat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.n, self.d)
    }
}