pub fn run() {
    let my_closure = |value| println!("This is a closure. Print: {}", value);

    let x = 1;
    let y = 2;

    let sum = |input: usize| println!("X + Y + INPUT = {}", x + y + input);

    my_closure("Asd");

    let generate_adder = |input: usize| {
        let adder = move |adder_input: usize| {
            println!("{} + {} = {}", input, adder_input, input + adder_input)
        };
        adder
    };

    sum(3);

    let add_7 = generate_adder(7);

    add_7(3);
    add_7(5);
}
