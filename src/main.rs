fn main () {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
/*
#![allow(unused_variables)]
fn main() {
    let r;
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        r = &x;               // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
}
*/
