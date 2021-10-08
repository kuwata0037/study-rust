#[cfg(test)]
mod tests {
    use std::{error::Error, sync::mpsc, thread};

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
}
