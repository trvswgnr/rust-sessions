use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::thread;

fn old() {
    let mut counter = 0;
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    let handle_1 = thread::spawn(move || {
        counter += 1;

        println!("counter in handle 1: {}", counter);
    });

    handles.push(handle_1);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter: {}", counter);
}

fn new() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    let counter_clone = counter.clone();
    let handle_1 = thread::spawn(move || {
        let mut num = counter_clone.lock().unwrap();
        *num += 3;
    });

    let counter_clone_2 = Arc::clone(&counter);
    let handle_2 = thread::spawn(move || {
        let mut num = counter_clone_2.lock().unwrap();
        *num += 1;
    });
    handles.push(handle_1);
    handles.push(handle_2);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter: {}", *counter.lock().unwrap());
}

fn main() {
    new();
    let x = transform(50i64);
}

trait Foo {
    fn baz() {
        // do
    }
}

trait Bar {
    fn baz() {}
}

struct Things;

fn transform<T: ToString>(n: T) -> String {
    n.to_string()
}

trait Thing {
    fn transform() {}
}

impl<T> Thing for T
where
    Option<T>: Debug,
{
    fn transform() {}
}
