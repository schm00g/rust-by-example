fn main(){
    // variable types can be annotated
    let logical: bool = true;

    let a_float: f64 = 1.0; // regular annotation
    let an_integer = 5i32; // suffix annotation

    // or default will be used
    let default_float = 1.6; // `f64`
    let default_integer = 7; //  `i32`

    // a type can also be inferred from context
    let mut inferred_type = 12;// i64
    inferred_type = 12341343243i64;

    // a mutable
    let mut mutable = 12;
    mutable = 21;

    mutable = true; // ERROR -> ***type*** of mutable CANNOT be changed
    
    // but we can use SHADOWING to overwrite
    let mutable = true;
    println!("{}", mutable);
}