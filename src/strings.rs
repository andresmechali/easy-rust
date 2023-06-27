use std::cell::Cell;
use std::fmt::{Display, Formatter};

pub fn run() {
    println!("{}", return_static_reference());

    let mut some_adventurer = Adventurer {
        first_name: "John",
        last_name: "Doe",
        hit_points: 100,
    };

    some_adventurer.take_damage();
    some_adventurer.take_damage();

    let phone = Phone {
        company: String::from("Apple"),
        price: Cell::new(999),
        on_sale: Cell::new(false),
    };

    phone.price.set(900);
    phone.on_sale.set(true);

    println!("phone -> {}", phone);
}

fn return_static_reference() -> &'static str {
    "static string"
}

struct City<'citylifetime> {
    name: &'citylifetime str,
    date_founded: u32,
}

fn returns_references<'a>(input_a: &'a str, input_b: &'a str) -> &'a str {
    if input_a.len() > 2 {
        return input_a;
    }
    input_b
}

struct Adventurer<'a, 'b> {
    first_name: &'a str,
    last_name: &'b str,
    hit_points: u32,
}

impl Adventurer<'_, '_> {
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!(
            "{} {} has {} hit points left",
            self.first_name, self.last_name, self.hit_points
        );
    }
}

struct Phone {
    company: String,
    price: Cell<u32>,
    on_sale: Cell<bool>,
}

impl Display for Phone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "brand: {}, price: {}, on sale: {}",
            self.company,
            self.price.get(),
            self.on_sale.get()
        )
    }
}
