pub fn run() {
    let mut doubler = Doubler::default();

    while let Some(current_value) = doubler.next() {
        println!("{:?}", current_value);
    }
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
