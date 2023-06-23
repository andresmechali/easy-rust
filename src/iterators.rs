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

    println!(
        "odd numbers squared: {:?}",
        some_numbers
            .clone()
            .into_iter()
            .filter_map(|num| {
                match num % 2 {
                    0 => None,
                    _ => Some(num * num),
                }
            })
            .collect::<Vec<_>>()
    );

    let user_input = vec!["8.9", "two", "3", "one"];

    let actual_numbers = user_input
        .into_iter()
        .filter_map(|x| x.parse::<f32>().ok())
        .collect::<Vec<f32>>();

    println!("actual numbers: {:?}", actual_numbers);

    let some_vec = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    println!("and_then: {:?}", some_vec.get(2).and_then(|x| x.get(1)));

    let boolean_vec = vec![vec![Some(1), None, Some(2)], vec![None, None, Some(3)]];

    for i in 0..boolean_vec[0].len() {
        let and_result = boolean_vec[0][i].and(boolean_vec[1][i]);
        println!(
            "{:?} and {:?} = {:?}",
            boolean_vec[0][i], boolean_vec[1][i], and_result
        );
    }

    let some_string = "some string";
    println!(
        "{} contains {} = {}",
        some_string,
        'e',
        in_char_vec(&some_string.chars().collect::<Vec<char>>(), 'e')
    );
    println!(
        "{} contains {} = {}",
        some_string,
        'z',
        in_char_vec(&some_string.chars().collect::<Vec<char>>(), 'z')
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

fn in_char_vec(char_vec: &Vec<char>, check: char) -> bool {
    char_vec.iter().any(|x| *x == check)
}
