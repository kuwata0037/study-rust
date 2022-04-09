use std::{
    error::Error,
    sync::{mpsc, Arc, Mutex},
    thread, vec,
};

#[test]
fn test_mpsc() -> Result<(), Box<dyn Error + 'static + Send + Sync>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("test1".to_string()).unwrap();
        tx.send("test2".to_string()).unwrap();
    });

    assert_eq!(rx.recv()?, "test1".to_string());
    assert_eq!(rx.recv()?, "test2".to_string());

    Ok(())
}

#[test]
fn test_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            *counter.lock().unwrap() += 1;
        });
        handlers.push(handler);
    }

    handlers.into_iter().for_each(|handler| {
        handler.join().unwrap();
    });

    assert_eq!(*counter.lock().unwrap(), 10);
}
