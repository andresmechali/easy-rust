use std::collections::HashMap;

pub fn run() {
    let mut doubler = Doubler::default();

    while let Some(current_value) = doubler.next() {
        println!("{:?}", current_value);
    }

    let some_numbers = vec![0, 1, 2, 3, 4, 5];
    let some_words = vec!["zero", "one", "two", "three", "four"];

    let number_word_hashmap = some_numbers
        .into_iter()
        .zip(some_words.into_iter())
        .collect::<HashMap<i32, &str>>();

    println!(
        "key: {}, value: {}",
        1,
        number_word_hashmap.get(&1).unwrap()
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
