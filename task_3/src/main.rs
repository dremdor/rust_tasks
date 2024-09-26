use std::{sync::mpsc, thread};

fn square(x: isize) -> isize{
    x * x
}

fn main() {
    const N: usize = 100; 
    let mut arr: [isize; N] = [0; N];

    // инициализация массива 
    for (i, item) in arr.iter_mut().enumerate().take(N) {
        *item = (i + 1) as isize;
    }

    // инициализация отправителя и получателя
    let (tx, rx) = mpsc::channel();

    // дожидание выполнения каждого потока 
    let mut handles = vec![];

    // передача владение item в новые потоки
    for &item in arr.iter() {
        // клонирование передатчика
        let tx1 = tx.clone();

        let handle = thread::spawn(move || {
            let value = square(item);
            tx1.send(value).unwrap()
        });

        handles.push(handle);
    }
    // закрытие основного передатчика, чтобы прекратить ожидания новых
    // сообщений получателем
    drop(tx);
    // использование join() гарантирует что порожденные потоки std::thread
    // завершатся раньше меина
    for handle in handles {
        handle.join().unwrap();
    }
    // подсчет суммы после вычислений 
    let mut sum = 0;

    for item in rx {
        sum += item;
    }

    println!("{sum}");
}
