struct Subject {
    name: String,
}

impl Subject {
    fn print(&self) {
        println!("{}", self.name)
    }
}

struct Observer<'a> {
    // target: Vec<&'a Subject>,
    target: &'a Subject,
}

impl<'a> Observer<'a> {
    fn watch(&self) {
        self.target.print();
    }
}

fn main() {
    let s1 = Subject { name: "abc".to_string() };
    s1.print();
    let s2 = Subject { name: "def".to_string() };
    s2.print();

    let o = Observer{ target: &s1};
    o.watch();
}