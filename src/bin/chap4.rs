fn main() {
    println!("This is Chapter  Four");
    print_padovan();
}

fn print_padovan() -> () {
    let mut padovan: Vec<i32> = vec![1, 1, 1];
    for n in 3..10 {
        let next = padovan[n - 3] + padovan[n - 2];
        padovan.push(next)
    }

    println!("The result of P(1..10) = {:?}", padovan)
}