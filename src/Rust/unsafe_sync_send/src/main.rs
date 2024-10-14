use std::{sync::Arc, thread};

struct NotThreadSafe {
    data: *mut i32,
}

unsafe impl Sync for NotThreadSafe {}
unsafe impl Send for NotThreadSafe {}

fn main() {
    let mut x = 0;

    let not_thread_safe = NotThreadSafe {
        data: &mut x as *mut i32,
    };

    let shared_data = Arc::new(not_thread_safe);

    let thread1 = {
        let shared_data = Arc::clone(&shared_data);
        thread::spawn(move || unsafe {
            for _ in 1..1001 {
                *shared_data.data += 1;
                println!("Thread 1: x = {}", *shared_data.data);
            }
        })
    };

    let thread2 = {
        let shared_data = Arc::clone(&shared_data);
        thread::spawn(move || unsafe {
            for _ in 1..1001 {
                *shared_data.data += 1;
                println!("Thread 2: x = {}", *shared_data.data);
            }
        })
    };

    // Wait for both threads to finish.
    thread1.join().unwrap();
    thread2.join().unwrap();

    // Print final value of x.
    println!("Final value of x: {}", x);
}
