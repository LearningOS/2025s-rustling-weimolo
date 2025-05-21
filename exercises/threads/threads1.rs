// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

// AI
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    
    // Spawn 10 threads
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis() // Return the elapsed time in milliseconds
        }));
    }

    let mut results: Vec<u128> = vec![];

    // Collect the results from all the threads
    for handle in handles {
        match handle.join() {
            Ok(result) => results.push(result), // On success, add the result to results
            Err(_) => panic!("Oh no! A thread panicked!"),
        }
    }

    // Ensure we got results from all threads
    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    
    // Print the time taken by each thread
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
