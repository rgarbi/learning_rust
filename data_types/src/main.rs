use std::mem;

fn main() {
    let eight_bit: u8 = get_eight_bit();
    println!("eight_bit = {}", eight_bit);

    let mut mut_eight_bit: i8 = -128;
    mut_eight_bit = mut_eight_bit + 1;
    println!("mut_eight_bit = {}", mut_eight_bit);

    let c = 123456789;
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));

    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    let character: char = 'x';
    println!("{} is a char, size = {} bytes", character, mem::size_of_val(&character));

    let e = 2.5;
    println!("{}, size = {} bytes", e, mem::size_of_val(&e));

    let boolean: bool = false;
    println!("{} is a char, size = {} bytes", boolean, mem::size_of_val(&boolean));
}


fn get_eight_bit() -> u8 {
    return 255;
}