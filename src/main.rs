use std::ops::Add;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    //I tend to use concurrent programming and parallel programming interchangeably. However,
    // apparently they have slightly different meanings. Concurrent programming simply means that
    // different parts of a program execute independently. Parallel programming is when different
    // parts of a program run at the same time. So I suppose all parallel programs are also
    // concurrent programs, but all concurrent programs are necessarily parallel. For the Rust book,
    // concurrent is used instead of concurrent and/or parallel.

    using_threads_to_run_code_simultaneously();
    using_message_passing_to_transfer_data_between_threads();
    shared_state_concurrency();
    extensibility_concurrency_with_the_sync_and_send_traits();
}

fn using_threads_to_run_code_simultaneously() {
    //Rust uses a 1:1 thread model. So a single thread in Rust is an OS thread. I suppose this
    // would be different than say Kotlin which uses coroutines.

    //To create a thread in this language, use thread::spawn. To sleep a thread, use thread::sleep.
    let first_handle = thread::spawn(|| {
        for i in 0..10 {
            println!("First thread i: {i}");
            thread::sleep(Duration::from_millis(10));
        }
    });

    let second_handle = thread::spawn(|| {
        for i in 0..10 {
            println!("Second thread i: {i}");
            thread::sleep(Duration::from_millis(10));
        }
    });

    first_handle.join().expect("First thread crashed");
    second_handle.join().expect("Second thread crashed");


    let my_str = String::from("moved string");

    //In order for a thread to properly use a value, it must move in the value to the thread. The
    // `move` keyword is used for this purpose.
    let move_handle = thread::spawn(move || {
        println!("my_str: {my_str}");
    });

    //The below line is invalid because my_str was moved into the thread.
    // println!("my_str: {my_str}");

    move_handle.join().expect("move_handle crashed");
}

fn using_message_passing_to_transfer_data_between_threads() {
    //Rust provides channels to communicate between threads. I have used channels in Kotlin, but
    // I don't have nearly as intuitive understanding of them as I do of memory sharing in general.
    // Channels only work in one direction. They have two halves, the transmitter which transmits
    // data and the receiver which receives data.

    //The abbreviations `tx` and `rx` are traditionally used in many fields for transmitter and
    // receiver respectively.
    let (tx, rx) = mpsc::channel();

    //The transmitter can be cloned if multiple are needed. The receiver cannot be cloned.
    let tx1 = tx.clone();

    thread::spawn(move || {
        let hello = "hello";

        //If the receiver has already been dropped, this will return an error.
        tx.send(hello).expect("Failed to send");
    });

    thread::spawn(move || {
        let multiple = vec![
            "channels",
            "are",
            "fun",
        ];

        for val in multiple {
            tx1.send(val).expect("Failed to send on tx1");
            thread::sleep(Duration::from_millis(300));
        }
    });

    //There is a blocking and a non blocking method here with the receiver. try_recv() is
    // non-blocking and recv() is blocking.
    // let received = rx.recv().expect("Failed to receive");
    // println!("received: {received}");

    //This seems to be syntactic sugar to block until everything is received (I assume it ends when
    // all transmitters have gone out of scope).
    for val in rx {
        println!("val: {val}");
    }
}

fn shared_state_concurrency() {
    //This section is about shared memory. This is in contrast to the channels used above.

    //The mutex here isn't an object that I lock and unlock like I think about in most languages.
    // It is essentially a wrapper around a value. So the mutex itself works a bit differently
    // than I am used to.
    let m = Mutex::new(5);
    {
        //This will panic if the lock is already held by this thread. Otherwise it will block until
        // it can acquire the lock.
        //This returns a MutexGuard which is a smart pointer and represents the lock. When it goes
        // out of scope, the mutex will unlock.
        let mut value = m.lock().unwrap();
        *value += 1;
    }

    println!("m: {:?}", m);

    //The way to use a mutex across multiple threads is with the Arc<T> object. This is the same
    // as the Rc<T> object except that internally it uses an atomic reference counter while Rc<T>
    // does not.
    let shared = Arc::new(Mutex::new(String::from("1")));
    let mut threads = vec![];

    for i in 0u8..10u8 {
        let next_mutex = Arc::clone(&shared);
        threads.push(
            thread::spawn(move || {
                let mut string = next_mutex.lock().unwrap();
                string.push(('a' as u8 + i) as char);
            })
        );
    }

    for thread in threads {
        thread.join().expect("Failed at threads");
    }

    let final_state = shared.lock().unwrap();
    println!("final_state: {}", *final_state);

    //Conceptually it is worth noting that Mutex<T> along with Arc<T> has a lot of similarities to
    // RefCell<T> along with Rc<T>. Mutex<T> has interior mutability just like RefCell<T>, it also
    // has some of the same problems. Just like RefCell<T> has reference cycles, Mutex<T> has
    // deadlocks.
}

fn extensibility_concurrency_with_the_sync_and_send_traits() {
    //The language Rust itself has very few concurrency features. Most of the concurrency features
    // are actually part of the standard library. This is possible because Rust two gives Traits
    // that allow for implementing these features.
    // 1) std::marker::Send; This indicates that the type can be transferred between threads. This
    //  is implemented for almost all Rust built in types, Rc<T> is an exception.
    // 2) std::marker::Sync; This indicates that the type can be referenced from multiple threads
    //  at the same time. For example, `let m = 5` means that &m can be used from multiple threads
    //  simultaneously.

    //It is also worth noting that if a type is comprised of types implementing Send and/or Sync,
    // the type will also be Send and/or Sync.

    //It seems to be rather difficult to implement Send or Sync myself and it requires unsafe Rust
    // code. This makes sense because I can't see any way that the borrow checker could enforce
    // that say a custom shared lock could work.
}
