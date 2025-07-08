use fatty_acid_macro::fatty_acid;
use polars::{chunked_array::builder::AnonymousOwnedListBuilder, prelude::*};

/// Fatty acid bounds column name
pub const BOUNDS: &str = "Bounds";
/// Fatty acid carbon column name
pub const CARBON: &str = "Carbon";
/// Fatty acid column name
pub const FATTY_ACID: &str = "FattyAcid";
/// Fatty acid bound index column name
pub const INDEX: &str = "Index";
/// Fatty acid bound parity column name
pub const PARITY: &str = "Parity";
/// Fatty acid bound triple column name
pub const TRIPLE: &str = "Triple";

#[macro_export]
macro_rules! data_type {
    (FATTY_ACID) => {
        DataType::Struct(vec![field!(CARBON), field!(BOUNDS)])
    };
    (BOUNDS) => {
        DataType::List(Box::new(data_type!(BOUND)))
    };
    (BOUND) => {
        DataType::Struct(vec![field!(INDEX), field!(PARITY), field!(TRIPLE)])
    };
}

#[macro_export]
macro_rules! field {
    (CARBON) => {
        Field::new(PlSmallStr::from_static(CARBON), DataType::UInt8)
    };
    (BOUNDS) => {
        Field::new(PlSmallStr::from_static(BOUNDS), data_type!(BOUNDS))
    };
    (INDEX) => {
        Field::new(PlSmallStr::from_static(INDEX), DataType::Int8)
    };
    (PARITY) => {
        Field::new(PlSmallStr::from_static(PARITY), DataType::Boolean)
    };
    (TRIPLE) => {
        Field::new(PlSmallStr::from_static(TRIPLE), DataType::Boolean)
    };
}

// fatty_acid!(C18 { });
// fatty_acid!(C18 { 9 => DC });
// fatty_acid!(C18 { 9 => DC, 12 => DT });
// fatty_acid!(C18 { 9 => DC, 12 => DT, 15 => D });

#[test]
fn test() -> PolarsResult<()> {
    let c18u3dc9dt12d15 = fatty_acid!(C18 { 9 => DC, 12 => DT, 15 => D })?;
    println!("c18u3dc9dt12d15: {c18u3dc9dt12d15}");
    // let c18u0 = fatty_acid!(C18 {});
    // println!("c18u0: {c18u0:?}");
    // let c18u3dc9dt12d15 = fatty_acid!(C18 { 9 => DC, 12 => DT, 15 => D });
    // println!("c18u3dc9dt12d15: {c18u3dc9dt12d15:?}");
    let c18u3dcn9dtn6dn3 = fatty_acid!(C18 { -9 => DC, -6 => DT, -3 => D })?;
    println!("c18u3dcn9dtn6dn3: {c18u3dcn9dtn6dn3}");
    // let c18u4dc0dt0d0t0 = fatty_acid!(C18 { 0 => D, 0 => DC, 0 => DT, 0 => T });
    // println!("c18u4dc0dt0d0t0: {c18u4dc0dt0d0t0:?}");
    Ok(())
}
