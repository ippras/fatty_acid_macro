use fatty_acid_macro::fatty_acid;
use polars::prelude::*;

// fatty_acid!(C18U2DC9DC12);
// fatty_acid!(18:0);
// fatty_acid!(18:1 { 9 => DC });
// fatty_acid!(18:2 { 9 => DC, 12 => DC });

#[test]
fn test() {
    let c18u0 = fatty_acid!(C18U0);
    println!("C18U0: {c18u0:?}");
    let c18u3dc9dt12d15 = fatty_acid!(C18U3DC9DT12D15);
    println!("C18U3DC9DT12D15: {c18u3dc9dt12d15:?}");
    let c18u4dc0dt0d0t0 = fatty_acid!(C18U4DC0DT0D0T0);
    println!("C18U4DC0DT0D0T0: {c18u4dc0dt0d0t0:?}");
    // println!("C18U2DC9DC12: {C18U2DC9DC12:?}");
}
