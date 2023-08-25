struct Subject {
    name: String,
}

impl Subject {
    fn print(&self) {
        println!("{}", self.name)
    }
}

struct Observer<'a> {
    target: Vec<&'a Subject>,
}

impl<'a> Observer<'a> {
    fn new() -> Self {
        Self{ target: Vec::new() }
    }
    fn add(&mut self, sub: &'a Subject) {
        self.target.push(sub);
    }
    fn watch(&self) {
        for t in self.target.iter() {
            t.print();
        }
    }
}

fn main() {
    let mut o = Observer::new();

    // {
    let s1 = Subject { name: "abc".to_string() };
    let s2 = Subject { name: "def".to_string() };
    o.add(&s1);
    o.add(&s2);
    // }
    o.watch();
}