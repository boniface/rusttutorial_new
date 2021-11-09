fn main() {
    refs();
}

fn refs (){
    let r ;
    {
        let x = 1;
        r = &x;
        assert_eq!(*r,1)
    }

}