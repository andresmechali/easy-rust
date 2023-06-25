pub fn run() {
    println!("{}", return_static_reference());
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
