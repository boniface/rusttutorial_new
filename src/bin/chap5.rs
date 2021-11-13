fn main() {
    let x = String::from(" Hello Chatiba");
    let mut s = String::from(" Hello Mutabele ");
    println!(" The S Value is {}", s);
    println!(" The Result is {}", x);
    take(&mut s);
    println!(" The Result is {}", s);

}

fn take(va: &mut String)-> (){

    println!(" This is Borrowed {:?}", va.to_owned()+"hello");
    println!(" The Borrowd Balue  {:?}", va);
}
