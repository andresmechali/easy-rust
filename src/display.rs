use std::fmt::{Display, Formatter};

pub struct Cat {
    pub name: String,
    pub age: u8,
}

impl Display for Cat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The cat's name is {} and the age is {}.",
            self.name, self.age
        )
    }
}

pub fn run() {
    let cat = Cat {
        name: "Garfield".to_string(),
        age: 4,
    };

    println!("{}", cat);
}
