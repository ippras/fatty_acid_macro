use fatty_acid_macro::fatty_acid;
use polars::prelude::*;

// fatty_acid!(C18 { });
// fatty_acid!(C18 { 9 => DC });
// fatty_acid!(C18 { 9 => DC, 12 => DT });
// fatty_acid!(C18 { 9 => DC, 12 => DT, 15 => D });

#[test]
fn test() {
    let c18u0 = fatty_acid!(C18 {});
    println!("c18u0: {c18u0:?}");
    let c18u3dc9dt12d15 = fatty_acid!(C18 { 9 => DC, 12 => DT, 15 => D });
    println!("c18u3dc9dt12d15: {c18u3dc9dt12d15:?}");
    let c18u3dcn9dtn6dn3 = fatty_acid!(C18 { -9 => DC, -6 => DT, -3 => D });
    println!("c18u3dcn9dtn6dn3: {c18u3dcn9dtn6dn3:?}");
    let c18u4dc0dt0d0t0 = fatty_acid!(C18 { 0 => D, 0 => DC, 0 => DT, 0 => T });
    println!("c18u4dc0dt0d0t0: {c18u4dc0dt0d0t0:?}");
}
