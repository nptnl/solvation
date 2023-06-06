pub mod comp;
pub mod prim;
pub mod trig;
pub mod rat;

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

    ArcHypSine,
    ArcHypCosine,
    ArcHypTangent,
    ArcHypCotangent,
    ArcHypSecant,
    ArcHypCosecant,

    Sqrt,
    
}