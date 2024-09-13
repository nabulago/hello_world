fn primitive_examples(){

    // Variables can be type annotated.
    let logical: bool = true;
    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    // or a default will be used.
    let default_float = 3.0;    // 'f64'
    let default_int = 7;        // 'i32'

    // A type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 1234567891i64;

    // A mutable variable'svalue can be changed.
    let mut mutable = 12; // Mutable 'i32'
    mutable = 21;

    //Error! The type of a variable can't be changed.
    mutable = True;

    // Variables can be overwritten with shadowing    
    let mutable = True;

    println!("{} {} {}",logical, a_float, an_integer);
    println!("Default float{}", default_float);
    println!("Default int{}", default_int);

}

fn main() {
    println!("Hello, world!");

    primitive_examples()

}
