// We include these so we can have things like
// fn encode(self) -> [u8; Self::ENCODED_LENGTH];
// defined within the Fq trait
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod fp2_gen;
pub mod fp_gen;
pub mod fq;
pub mod test_macros;
pub mod utils64;
