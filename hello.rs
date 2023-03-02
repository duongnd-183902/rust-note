#![allow(dead_code)]    
struct Point{
    x: u32,
    y: u32,
}
enum Status{
    Poor,
    Rich,
}
fn main() {
    let duongnd = "DUONG";
    println!("{duongnd}, {0:<5}, {1:x} , {duongnd} , {rust}",1,200,rust = {5});
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    let x: u32 = 7;
    let y: u32 = 9;
    let point: Point = Point{x: x, y: y};
    let q : Point = Point{1,..point}; 
    use crate::Status::{Poor,Rich};
    let poor = Poor;
    match poor {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // This binding *shadows* the previous binding
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);

}
