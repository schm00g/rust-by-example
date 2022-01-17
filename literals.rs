fn main(){
    // integer addition
    println!("1 + 5 = {}", 1i32 + 5);

    // integer subtraction
    println!("10 - 3 = {}", 5i32 + 2);

    println!("true and false is {}", true && false);
    println!("true or false is {}", true || false);
    println!("NOT true is {}", !true);

    // Integers can, alternatively, be expressed using hexadecimal, octal or binary 
    // notation using these prefixes respectively: 0x, 0o or 0b.

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    
    // underscores improve readability
    println!("One million is written as {}", 1_000_000);
}