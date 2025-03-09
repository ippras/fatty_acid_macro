use fatty_acid_macro::fatty_acid;

#[derive(Clone, Copy, Debug)]
struct FattyAcid<const N: usize>([&'static str; N]);

fatty_acid! {
    pub C18U0;
}
// fatty_acid!(C18U2DC9DC12);
// fatty_acid!(18:0);
// fatty_acid!(18:1 { 9 => DC });
// fatty_acid!(18:2 { 9 => DC, 12 => DC });

#[test]
fn test() {
    println!("C18U0: {C18U0:?}");
    // println!("C18U2DC9DC12: {C18U2DC9DC12:?}");
}
