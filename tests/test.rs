use fatty_acid_macro::fatty_acid;
use polars::prelude::*;

/// Fatty acid carbon column name
pub const CARBON: &str = "Carbon";
/// Fatty acid column name
pub const FATTY_ACID: &str = "FattyAcid";
/// Fatty acid index column name
pub const INDEX: &str = "Index";
/// Fatty acid indices column name
pub const INDICES: &str = "Indices";
/// Fatty acid parity column name
pub const PARITY: &str = "Parity";
/// Fatty acid triple column name
pub const TRIPLE: &str = "Triple";

#[macro_export]
macro_rules! data_type {
    (FATTY_ACID) => {
        DataType::Struct(vec![field!(CARBON), field!(INDICES)])
    };
    (INDICES) => {
        DataType::List(Box::new(data_type!(INDEX)))
    };
    (INDEX) => {
        DataType::Struct(vec![field!(INDEX), field!(TRIPLE), field!(PARITY)])
    };
}

#[macro_export]
macro_rules! field {
    (CARBON) => {
        Field::new(PlSmallStr::from_static(CARBON), DataType::UInt8)
    };
    (INDICES) => {
        Field::new(PlSmallStr::from_static(INDICES), data_type!(INDICES))
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
    let c18u3c9t12o15 = fatty_acid!(C18 { 9 => C, 12 => T, 15 => O })?;
    println!("c18u3c9t12o15: {c18u3c9t12o15}");
    // let c18u0 = fatty_acid!(C18 {});
    // println!("c18u0: {c18u0:?}");
    // let c18u3dc9dt12d15 = fatty_acid!(C18 { 9 => DC, 12 => DT, 15 => D });
    // println!("c18u3dc9dt12d15: {c18u3dc9dt12d15:?}");
    let c18u3_c9_t6_o3 = fatty_acid!(C18 { -9 => C, -6 => T, -3 => O })?;
    println!("c18u3_c9_t6_o3: {c18u3_c9_t6_o3}");
    // let c18u4dc0dt0d0t0 = fatty_acid!(C18 { 0 => D, 0 => DC, 0 => DT, 0 => T });
    // println!("c18u4dc0dt0d0t0: {c18u4dc0dt0d0t0:?}");
    Ok(())
}
