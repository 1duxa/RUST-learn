use std::fmt;
struct HelloWorld {
    name: i32,
}

impl fmt::Debug for HelloWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Hello world |")
            .field("x", &self.name)
            .finish()
    }
}
impl fmt::Display for HelloWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hello his name is({})", self.name)
    }
}

impl HelloWorld {
    fn printdata(&self) {
        println!("{}", self);
    }
}

fn main() {
    let a = HelloWorld { name: 8 };
    a.printdata()
}
