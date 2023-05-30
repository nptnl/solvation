pub mod repl;
pub mod preset;
pub mod math;

fn main() {
    // repl::roll();
    println!("{:?}", math::rat::Rat::new(3,4))
}