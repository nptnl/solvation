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


pub static PRE_DO: [&str; 5] = [
"σ.tot = 0",
"σ.x = 0",
"σ.dx = 0.00001",
"def RMN(f, x1, x2) σ.x = x1, [ σ.tot = σ.tot + f(σ.x) * σ.dx, σ.x = σ.x + σ.dx, :(σ.x > 1, ∇) ], ans = σ.tot",
"def LD(f, x) ans = ( f(x + σ.dx) - f(x) ) / dx",
];