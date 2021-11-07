fn main() {
    println!("This is a a test bin");

    assert_eq!(10_i8 as u16,10_u16); // in range
    assert_eq!( 2525_u16 as i16, 2525_i16); // in range
    assert_eq!( -1_i16 as i32,-1_i32); // sign-extended
    assert_eq!(65535_u16 as i32, 65535_i32); // zero-extended
    ops_on_types();
}

fn ops_on_types(){
    // println!("{}", (-4).abs()); Will not compile
    println!("{}", -4_i32.abs());
    println!("{}", (-4_i32).abs());
    println!("{}", i32::abs(-4));
}