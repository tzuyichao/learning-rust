use std::thread;
use std::time::Duration;

fn main() {
    let no_move_thread = start_no_shared_data_thread();

    for _ in 0..10 {
        print!(":");
    }

    println!("Waiting for the thread to finish ... {:?}", no_move_thread.join());
}

fn start_no_shared_data_thread() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        println!("Waiting for three seconds.");
        thread::sleep(Duration::from_secs(3));
        println!("Done.");
    })
}
