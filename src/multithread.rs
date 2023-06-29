use std::sync::Mutex;

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
}
