pub fn run() {
    println!("{}", return_static_reference());
}

fn return_static_reference() -> &'static str {
    "static string"
}
