use std::any::type_name;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>(); //Checking data type need to add function as not works directly
}

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
    println!("First inferred type variable as need mutable {}",inferred_type);
    inferred_type = 1234567891i64;
    println!("First inferred type variable as after mutaion : {}",inferred_type);
    println!("First inferred type variable as after mutaion dataatype: {}",type_of(&inferred_type));

    // A mutable variable'svalue can be changed.
    let mut mutable = 12; // Mutable 'i32'
    println!("Mutable for the integer 32 {}",mutable);
    mutable = 21;
    println!("Mutable for the integer 32 {}",mutable);
    // ----------------------------------------------
    //Error! The type of a variable can't be changed.
    // mutable = true; // true should be in lower case
    // Commented this part as the mutable can change values 
    // but we are assigning the boolean value 
    // ----------------------------------------------
    
    // Variables can be overwritten with shadowing    
    let mutable = true;
    println!("Mutable for the Boolean redefined {}",mutable);

    println!("{} {} {}",logical, a_float, an_integer);
    println!("Default float{}", default_float);
    println!("Default int{}", default_int);

}

fn main() {
    println!("Hello, world!");

    primitive_examples();

}
