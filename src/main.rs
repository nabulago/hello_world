fn primitive_examples(){

    // Variables can be type annotated.
    let logical: bool = true;
    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    println!(logical, a_float, an_integer);


}

fn main() {
    println!("Hello, world!");

    primitive_examples()

}
