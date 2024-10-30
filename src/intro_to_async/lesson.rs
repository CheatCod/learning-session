pub mod sync_and_send_trait {
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn sync_trait() {
        // The `Sync` trait is a marker trait that indicates that it is safe for the type implementing `Sync` to be shared between threads.

        // Very little types in Rust are `Sync` by default.
        // For example, `Rc<T>` is not `Sync` because it is not thread-safe.
        // Mutex<T> is `Sync` because it is thread-safe.
        let mutex = std::sync::Mutex::new(0);
        let mutex_ref = &mutex;
        let num = 0;
        let num_ref = &num;
        thread::scope(|s| {
            s.spawn(|| {
                let mut data = mutex_ref.lock().unwrap();
                println!("num_ref: {}", num_ref);
                *data += 1;
            });
            s.spawn(|| {
                let mut data = mutex_ref.lock().unwrap();
                println!("num_ref: {}", num_ref);
                *data += 1;
            });
        });
    }

    #[test]
    fn send_trait() {
        // The `Send` trait is a marker trait that indicates that it is safe for the type implementing `Send` to be sent from one thread to another.

        // Almost all types in Rust are `Send` by default.
        // `Rc<T>` is not `Send` because it is not thread-safe.

        // The following code will not compile because `Rc<T>` is not `Send`.

        // let rc = std::rc::Rc::new(0);
        // thread::spawn(|| {
        //     println!("{}", rc);
        // });
    }

    #[test]
    fn composite_types() {
        // The `Send` and `Sync` traits are automatically implemented for types that are composed entirely of `Send` and `Sync` types.
        // For example, the standard library's `Arc<T>` type is `Send` and `Sync` if `T` is `Send` and `Sync`.

        let rc = std::rc::Rc::new(0);
        let rc_mutex = std::sync::Mutex::new(rc);

        // won't compile

        // thread::spawn(|| {
        //     let data = rc_mutex.lock().unwrap();
        //     println!("{}", data);
        // });
    }

    #[test]
    fn unsafe_impl() {
        // The `Send` and `Sync` traits are unsafe to implement manually.
        // If you use unsafe impls, you are promising to Rust that your type is safe to use in a concurrent environment.

        // For example, the following code will compile, but it is not safe to use in a concurrent environment.

        #[derive(Debug)]
        struct UnsafeSend {
            rc: std::rc::Rc<i32>,
        }

        unsafe impl Send for UnsafeSend {}

        // now we can use UnsafeSend in a concurrent environment
        // note that this is not safe
        let rc = std::rc::Rc::new(0);
        let unsafe_send = UnsafeSend { rc };
        thread::spawn(move || {
            println!("{:?}", unsafe_send);
        });
    }
}

pub mod async_rust {
    // Rust's async implementation is runtime-agnostic, meaning you can use any async runtime you want.
    // An async runtime is a system that manages async tasks and decides when to run them.
    // The most popular async runtime in Rust is Tokio.
    // See main.rs on how #[tokio::main] works

    use std::time::Duration;

    async fn async_fn(num: i32) {
        println!("Hello, {num}!");
    }

    async fn background_task(id: i32) {
        println!("Background task {id} started");
        loop {
            println!("Background task {id} tick");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    async fn mock_http_req(message: &str, delay: u64) -> String {
        tokio::time::sleep(Duration::from_secs(delay)).await;
        format!("Response from server: {message}")
    }

    #[tokio::test]
    async fn futures() {
        // A future is a value that represents an asynchronous computation.
        // Futures are lazy, meaning they do not run until you explicitly ask them to.
        // All async functions return a future.

        // A future does not do anything until you call `.await` on it.
        // A future does not do anything until you call `.await` on it!
        // A future does not do anything until you call `.await` on it!!

        // The following code does not do anything because we did not call `.await` on the future
        async_fn(0);
    }

    #[tokio::test]
    async fn quiz() {
        // What will the following code print?
        let f1 = async_fn(1);
        async_fn(2).await;
        let f3 = async_fn(3);
        f1.await;
        f3.await;

        // Option 1: Hello, 1! Hello, 2! Hello, 3!
        // Option 2: Hello, 2! Hello, 1! Hello, 3!
        // Option 3: Hello, 2! Hello, 3! Hello, 1!
    }

    // !Note: run this with --nocapture to see the output
    // cargo test --package learning-session --bin learning-session -- intro_to_async::lesson::async_rust::spawning --exact --show-output --nocapture
    #[tokio::test]
    async fn spawning() {
        // If you want to run a future "in the background", you can use the `tokio::spawn` function.
        // The `tokio::spawn` function returns a `tokio::task::JoinHandle`, which is a future that you can optionally await on.
        tokio::spawn(background_task(1));
        tokio::spawn(background_task(2));
        tokio::spawn(background_task(3)).await.unwrap();
    }

    #[tokio::test]
    async fn join() {
        // The `tokio::join` function allows you to run multiple futures concurrently and wait for all of them to finish.
        // The `tokio::join` function returns a tuple of the results of the futures.
        let (result_1, result_2) =
            tokio::join!(mock_http_req("Hello", 1), mock_http_req("World", 2));
        println!("{result_1}");
        println!("{result_2}");
    }

    #[tokio::test]
    async fn select() {
        // The `tokio::select` macro allows you to run multiple futures concurrently and wait for the first one to finish.
        // The `tokio::select` macro returns a tuple of the index of the future that finished and the result of the future.
        // The unfinished future will be dropped.
        let (index, result) = tokio::select! {
            _ = mock_http_req("Hello", 1) => (0, "Hello"),
            _ = mock_http_req("World", 2) => (1, "World"),
        };
        println!("{index}");
        println!("{result}");
    }

    // !Note: run this with --nocapture to see the output
    // cargo test --package learning-session --bin learning-session -- intro_to_async::lesson::async_rust::do_not_block_the_runtime --exact --show-output --nocapture
    #[test]
    fn do_not_block_the_runtime() {
        // you may have noticed that we are using `tokio::time::sleep` to simulate a delay
        // instead of using `std::thread::sleep`

        // This is because `std::thread::sleep` will block the entire thread, which is not what we want in an async runtime.
        // To avoid blocking the runtime, you should use the async version of the function, if it exists.
        // This applies to all potentially blocking operations, such as file I/O, network I/O, etc.

        // To demonstrate, I will build a single-threaded async runtime using `tokio::runtime::Builder`
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            tokio::spawn(background_task(0));
            tokio::time::sleep(Duration::from_secs(5)).await;
            // this will block the runtime
            // std::thread::sleep(Duration::from_secs(5));
            println!("Done");
        });
    }
    fn blocking_fn() {
        // calculate the nth fibonacci number
        fn fib(n: u64) -> u64 {
            if n <= 1 {
                return n;
            }
            fib(n - 1) + fib(n - 2)
        }
        let n = 50;
        let result = fib(n);
        println!("fib({n}) = {result}");
    }
    // !Note: run this with --nocapture to see the output

    // cargo test --package learning-session --bin learning-session -- intro_to_async::lesson::async_rust::runtime_is_smart --exact --show-output --nocapture
    #[test]
    fn runtime_is_smart() {
        // The tokio runtime is smart enough to know when a future is blocking and will spawn it on a dedicated
        // "blocking" thread pool to avoid blocking future execution.

        // To do so, use `new_multi_thread` instead of `new_current_thread`
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            tokio::spawn(background_task(0));
            // this will block the runtime if the runtime is single-threaded
            // still a bad idea though
            blocking_fn();
            println!("Done");
        });
    }

    // !Note: run this with --nocapture to see the output
    // cargo test --package learning-session --bin learning-session -- intro_to_async::lesson::async_rust::running_blocking_code --exact --show-output --nocapture
    #[test]
    fn running_blocking_code() {
        // If you need to run blocking code in an async context, you can use the `tokio::task::spawn_blocking` function.
        // The `tokio::task::spawn_blocking` function will run the blocking code on a dedicated thread pool to avoid blocking the async runtime.

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .max_blocking_threads(2)
            .build()
            .unwrap();
        rt.block_on(async {
            tokio::spawn(background_task(0));
            tokio::task::spawn_blocking(|| {
                blocking_fn();
            })
            .await
            .unwrap();
            println!("Done");
        });
    }
}
