use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 10;

fn main() {
    let (tx, rx) = mpsc::channel();


    for i in 0..NTHREADS {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            thread_tx.send(i.pow(2)).unwrap();
            println!("поток {} завершён", i);
        });
    }
    drop(tx);

    let result = rx.iter().sum::<i32>();

    println!("Сумма квадратов чисел в масиве 1..{} равна {}", NTHREADS, result);
}