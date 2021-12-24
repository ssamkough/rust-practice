/**
 * Signed integer types: i8, i16, i32, i64 i128
 * Unsigned integer types: u8, u16, u32, u64, u128
 * Floating point types: f32, f64.
 */

fn main() {
    let is_cool: bool = true;
    println!("bool: {}", is_cool);
    let initial: char = 'S';
    println!("char: {}", initial);
    let age: u64 = 30;
    println!("u64: {}", age);
    let net_worth: f32 = 1.5;
    println!("f32: {}", net_worth);

    let price = 129;
    let tax = 23.22;
    let total = f64::from(price) + tax;
    println!("Total: {} + {} = {}", price, tax, total)
}