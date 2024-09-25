use std::thread;

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
    // дожидание выполнения каждого потока 
    let mut handles = vec![];
    // передача владение item в новые потоки
    for &item in arr.iter() {
        let handle = thread::spawn(move || {
            println!("{}", square(item));
        });

        handles.push(handle);
    }
    // использование join() гарантирует что порожденные потоки std::thread
    // завершатся раньше меина
    for handle in handles {
        handle.join().unwrap();
    }
}
