use std::sync::{Arc, Mutex};

fn main() {
    let v = Arc::new(Mutex::new(vec![1, 2, 3]));
    let v_clone = v.clone();

    let handle = std::thread::spawn(move || {
        let mut v = v_clone.lock().unwrap();
        *v.get_mut(0).unwrap() = 10;
    });

    let mut v = v.lock().unwrap();
    println!("{:?}", *v);

    handle.join().unwrap();
    let v = v.lock().unwrap();
    println!("{:?}", *v);
} 