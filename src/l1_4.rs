// Реализовать постоянную запись данных в канал (главный поток). 
// Реализовать набор из N воркеров, которые читают произвольные данные из канала и выводят в stdout. 
// Необходима возможность выбора количества воркеров при старте.
use std::sync::mpsc;
use std::thread;

fn main() {
    let n_workers = 5;
    let (tx, rx) = mpsc::channel();

    // Отправляем данные в канал
    for i in 0..10 {
        let data = format!("Data {}", i);
        tx.send(data).unwrap();
    }

    // Создаем воркеров
    let thread_tx = tx.clone();
    for i in 0..n_workers {
        let thread_rx = rx.clone();
        thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(data) => {
                        println!("Worker {}: {}", i, data);
                    }
                    Err(_) => break,
                }
            }
        });
    }

    // Отправляем данные в канал
    while let Ok(data) = rx.recv() {
        thread_tx.send(data).unwrap();
    }

    // Закрываем канал, чтобы завершить воркеры
    drop(tx);
}
