use std::sync::mpsc::channel;
use std::thread::spawn;

#[test]
fn test_channel() {
    let (sender1, receiver) = channel::<u8>();

    let sender2 = sender1.clone();

    spawn(move || {
        sender1.send(1).unwrap();
    });
    spawn(move || {
        sender2.send(2).unwrap();
    });

    let m1 = receiver.recv().unwrap();
    println!("main thread recv: {}", m1);

    let m2 = receiver.recv().unwrap();
    println!("main thread recv: {}", m2);
}
