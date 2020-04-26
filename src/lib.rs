use std::sync::mpsc::channel;
use std::sync::mpsc::sync_channel;
use std::thread;

#[inline]
pub fn mpscbench_async(n: u64) -> u64 {
    let (tx, rx) = channel();
    thread::spawn(move || {
        for _ in 0..n {
            tx.send(1).unwrap();
        }
    });

    let mut a = 0;
    for _ in 0..n {
        a += rx.recv().unwrap();
    }
    a
}

#[test]
fn test_mpscbench_async() {
    println!("{}", mpscbench_async(1000))
}

#[inline]
pub fn mpscbench_sync(n: u64) -> u64 {
    let (stx, srx) = sync_channel(0);
    thread::spawn(move || {
        for _ in 0..n {
            stx.send(1).unwrap();
        }
    });

    let mut b = 0;
    for _ in 0..n {
        b += srx.recv().unwrap();
    }
    b
}

#[test]
fn test_mpscbench_sync() {
    println!("{}", mpscbench_sync(1000))
}
