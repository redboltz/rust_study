#![allow(unused_variables)]
fn main() {
    //let r;
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;               // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
}
*/
