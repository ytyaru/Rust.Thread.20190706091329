/*
 * Rustのスレッド。
 * CreatedAt: 2019-07-06
 */
use std::thread;
use std::rc::Rc;
fn main() {
    let v = Rc::new(vec![1, 2, 3]);
    let handle = thread::spawn(|| {
//    let handle = thread::spawn(move || {
        let vv = Rc::clone(&v);
        vv.push(0);
        println!("vec: {:?}", v);
    });
    handle.join().unwrap();
    drop(v);
}
