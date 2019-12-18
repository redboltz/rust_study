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
