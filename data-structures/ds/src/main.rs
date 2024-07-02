use enum_swizzle::{swizzle, MultiVariateEnum};
mod enum_swizzle;


fn main() {
   let mut A = MultiVariateEnum::A { name: "HELLO WORLDS".to_string() };

    swizzle(&mut A);

    println!("{:#?}",A);

}
