use std::collections::HashMap;

pub fn run() {
    let mut doubler = Doubler::default();

    while let Some(current_value) = doubler.next() {
        println!("{:?}", current_value);
    }

    let some_numbers = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let some_words = vec!["zero", "one", "two", "three", "four"];

    let number_word_hashmap = some_numbers
        .clone()
        .into_iter()
        .zip(some_words.into_iter())
        .collect::<HashMap<i32, &str>>();

    println!(
        "key: {}, value: {}",
        1,
        number_word_hashmap.get(&1).unwrap()
    );

    println!("{:?}", "Hello".chars());

    for c in "Hello".char_indices() {
        println!("{:?}", c);
    }

    println!(
        "even numbers: {:?}",
        some_numbers
            .clone()
            .into_iter()
            .filter(|num| num % 2 == 0)
            .collect::<Vec<_>>()
    );

    println!(
        "odd numbers: {:?}",
        some_numbers.clone().retain(|num| *num == 1)
    );
}

impl Default for Doubler {
    fn default() -> Self {
        Self { value: 1 }
    }
}

struct Doubler {
    value: usize,
}

impl Iterator for Doubler {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.value < 1024 {
            self.value = self.value * 2;
            return Some(self.value);
        }
        None
    }
}
