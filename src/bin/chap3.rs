fn main() {
    println!("This is a a test bin");

    assert_eq!(10_i8 as u16, 10_u16); // in range
    assert_eq!(2525_u16 as i16, 2525_i16); // in range
    assert_eq!(-1_i16 as i32, -1_i32); // sign-extended
    assert_eq!(65535_u16 as i32, 65535_i32); // zero-extended
    ops_on_types();
    println!(" The Result is {:?}", checked_operations(100, 200));
    println!(" The Result For Division is {:?}", checked_division(120, -2));
    println!(" The Result For Product that fits {:?}", wrapping_multiplications(100, 200));
    println!(" The Result For Product that fits {:?}", wrapping_multiplications(500, 500));
    println!(" The Result For Product Wrapping is {:?}", wrapping_divisions(500, 500));
    println!(" The Result Left Shift {:?}", wrapping_shift(5, 17));
    println!(" The Result Left saturating  {:?}", saturating_addition(32_760_i16, 10));
    println!(" The Result Left sarating_sub {:?}", saturating_substracting(-32_760_i16, 10));
    println!(" The Overflowing Add {:?}", overflowing(127, 2));
    slices_vectors();
}

fn ops_on_types() {
    // println!("{}", (-4).abs()); Will not compile
    println!("{}", -4_i32.abs());
    println!("{}", (-4_i32).abs());
    println!("{}", i32::abs(-4));
}

fn checked_operations(num: u8, add: u8) -> Option<u8> {
    num.checked_add(add)
}

fn checked_division(num: i8, div: i8) -> Option<i8> {
    num.checked_div(div)
}

fn wrapping_multiplications(num: u16, value: u16) -> u16 {
    num.wrapping_mul(value)
}

fn wrapping_divisions(num: i16, value: i16) -> i16 {
    num.wrapping_mul(value)
}

fn wrapping_shift(num: i16, value: u32) -> i16 {
    num.wrapping_shl(value)
}

fn saturating_addition(num: i16, value: i16) -> i16 {
    num.saturating_add(value)
}

fn saturating_substracting(num: i16, value: i16) -> i16 {
    num.saturating_sub(value)
}

fn overflowing(num: i8, value: i8) -> (i8, bool) {
    num.overflowing_add(value)
}

fn slices_vectors() -> () {
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.0707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];
    let sv: &[f64] = &v;
    let sa: &[f64] = &a;
    printAr(sv);
}


fn printAr(n:&[f64]){
    for ma in n {
        print!("{}", ma);
    }
}



