pub mod basic_thread {
    use std::thread;
    use std::time::Duration;
    #[test]
    // Each program has at least one thread, which is the main thread that runs the `main` function
    fn main() {
        // `thread::spawn` creates a new thread and runs the closure passed to it
        let thread_1_handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the first spawned thread", i);
            }
        });

        let thread_2_handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the second spawned thread", i);
            }
        });

        // Wait for the spawned threads to finish
        thread_1_handle.join().unwrap();
        thread_2_handle.join().unwrap();
    }
}

pub mod move_semantics {
    use std::thread;
    #[test]
    fn main() {
        // The following code will not compile because rust cannot guarantee the vector will be valid for the lifetime of the spawned thread

        // In this case, we need to use the `move` keyword to transfer ownership of the vector to the spawned thread

        let v = vec![1, 2, 3];

        // let handle = thread::spawn(|| {
        //     println!("Here's a vector: {:?}", v);
        // });

        // handle.join().unwrap();
    }
}

pub mod sharing_data {
    use std::{
        sync::{Arc, Mutex},
        thread,
    };
    #[test]
    fn sharing_data_attempt_1() {
        // In safe Rust, the compiler will prevent you from writing code that leads to "data races" at compile time

        // let mut counter = 0;
        // let mut handles = vec![];
        // for _ in 0..10 {
        //     let handle = thread::spawn(|| {
        //         counter += 1;
        //     });
        //     handles.push(handle);
        // }
    }
    #[test]
    fn sharing_data_attempt_2() {
        // In safe Rust, the compiler will prevent you from writing code that leads to "data races" at compile time

        // let mut counter = 0;
        // let mut handles = vec![];
        // for _ in 0..10 {
        //     let handle = thread::spawn(move || {
        //         counter += 1;
        //     });
        //     handles.push(handle);
        // }
    }
    #[test]
    fn sharing_data_attempt_3() {
        // In safe Rust, the compiler will prevent you from writing code that leads to "data races" at compile time

        // let mut counter = 0;
        // let mut handles = vec![];
        // for _ in 0..10 {
        //     let counter_mut_ref = &mut counter;
        //     let handle = thread::spawn(move || {
        //         *counter_mut_ref += 1;
        //     });
        //     handles.push(handle);
        // }
    }
    #[test]
    fn sharing_data_attempt_4() {
        // In safe Rust, the compiler will prevent you from writing code that leads to "data races" at compile time

        // let mut counter = 0;
        // let mut handles = vec![];
        // for _ in 0..10 {
        //     let counter_mut_ref = &mut counter as *mut i32;
        //     let handle = thread::spawn(move || {
        //         *counter_mut_ref += 1;
        //     });
        //     handles.push(handle);
        // }
    }

    #[test]
    fn sharing_data_5_do_not_do_this() {
        struct UnsafeSendPointer {
            data: *mut i32,
        }

        impl UnsafeSendPointer {
            fn increment(&mut self) {
                unsafe {
                    *self.data += 1;
                }
            }
        }

        // Pinky promise to Rust that this struct is safe to send between threads
        // But we are lying, and we will suffer the consequences

        // We will explore Send and Sync in more detail in the next session
        unsafe impl Send for UnsafeSendPointer {}

        let mut counter = 0;
        let mut handles = vec![];
        for _ in 0..10 {
            let mut unsafe_counter = UnsafeSendPointer {
                data: &mut counter as *mut i32,
            };
            let handle = thread::spawn(move || {
                for _ in 0..1000 {
                    unsafe_counter.increment();
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // We expect the counter to be 10_000, but what will we actually get?
        println!("Counter: {}", counter);
    }

    #[test]
    fn sharing_data_with_arc_and_mutex() {
        let mut counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for _ in 0..10 {
            // clone the smart pointer each time we want to send it with a new thread
            let counter_cloned = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..1000 {
                    let mut num = counter_cloned.lock().unwrap();
                    *num += 1;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_counter = counter.lock().unwrap();
        println!("Counter: {}", *final_counter);
    }
}
