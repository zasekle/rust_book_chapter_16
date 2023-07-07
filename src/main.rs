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
