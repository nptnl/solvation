use crate::repl::{Bat, BinOp, Type};
use crate::math::comp::Comp;

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