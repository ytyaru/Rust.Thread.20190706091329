/*
 * Rustのスレッド。
 * CreatedAt: 2019-07-06
 */
use std::thread;
fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
//    let handle = thread::spawn(|| {
        println!("vec: {:?}", v);
    });
    drop(v); // 所有権がないため不可。error[E0382]: use of moved value: `v`
    handle.join().unwrap();
}
