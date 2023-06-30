use std::borrow::Cow;
use std::sync::{Mutex, RwLock};

pub fn run() {
    let my_mutex = Mutex::new(5);
    println!("my_mutex: {:?}", my_mutex); // Mutex { 5 }
    let mut mutex_changer = my_mutex.lock().unwrap();
    println!("my_mutex: {:?}", my_mutex); // Mutex { locked }
    println!("mutex_changer: {:?}", mutex_changer); // 5
    *mutex_changer = 6;
    println!("mutex_changer: {:?}", mutex_changer); // 6

    let my_mutex_2 = Mutex::new(1);
    *my_mutex_2.lock().unwrap() = 2;
    println!("my_mutex_2: {:?}", my_mutex_2); // Mutex { 2 }

    let my_book_1 = Book {
        name: Mutex::new("Name before"),
        author: Mutex::new("Author before"),
        sold: Mutex::new(100_000),
    };

    let mutex_changer = my_book_1.author.lock();
    drop(mutex_changer);

    if let Ok(mut mutex) = my_book_1.author.try_lock() {
        *mutex = "Author after";
    }

    println!("my_book_1: {:?}", my_book_1);

    for _ in 0..100 {
        *my_book_1.sold.lock().unwrap() += 1;
    }

    println!("my_book_1 sold: {:?}", my_book_1.sold);

    let my_book_2 = Book2 {
        title: RwLock::new("Book 2 title"),
        author: RwLock::new("Book 2 author"),
        sold: RwLock::new(50_000),
    };

    // read lock
    let _title = my_book_2.title.read().unwrap();
    let _author = my_book_2.author.read().unwrap();
    // write lock
    let mut sold = my_book_2.sold.write().unwrap();
    *sold = 60_000;
    drop(sold);

    println!("{:?}", my_book_2);

    for number in 1..=6 {
        match modulo_3(number) {
            Cow::Borrowed(message) => println!(
                "{} went in. The Cow is borrowed with this message: {}",
                number, message
            ),
            Cow::Owned(message) => println!(
                "{} went in. The Cow is owned with this message: {}",
                number, message
            ),
        }
    }
}

#[derive(Debug)]
struct Book<'a> {
    name: Mutex<&'a str>,
    author: Mutex<&'a str>,
    sold: Mutex<u32>,
}

#[derive(Debug)]
struct Book2<'a> {
    title: RwLock<&'a str>,
    author: RwLock<&'a str>,
    sold: RwLock<u32>,
}

fn modulo_3(input: u8) -> Cow<'static, str> {
    match input % 3 {
        0 => "Remainder is 0".into(),
        1 => "Remainder is 1".into(),
        remainder => format!("Remainder is {}", remainder).into(),
    }
}
