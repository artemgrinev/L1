use std::thread;

fn main() {
    let n: i32 = 10;

    for i in 0..n {
        thread::spawn(move || {
            let squared = i.pow(2);
            println!("{} в квадрате = {}", i, squared);
        });
    }
}