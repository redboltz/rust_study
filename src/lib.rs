use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

struct My {
    m1: i32,
    m2: i32,
}

impl Display for My {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.m1, self.m2)
    }
}

fn main() {
    let my1 = My{m1: 12, m2: 34};
    println!("{}", my1);
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn tc1() {
        let my1 = My {m1: 1, m2: 2};
        let my2 = My {m1: 3, m2: 4};
        assert_eq!(my1, my2);
    }
}
