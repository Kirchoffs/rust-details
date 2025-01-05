#[cfg(test)]
mod test {
    // Multiple producer, single consumer
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn channel_single_producer_demo() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    #[test]
    fn channel_single_producer_multiple_input_demo() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    #[test]
    fn channel_multiple_producer_demo() {
        let (alpha_tx, rx) = mpsc::channel();
        let beta_tx = alpha_tx.clone();

        thread::spawn(move || {
            let val = String::from("hi from alpha tx");
            alpha_tx.send(val).unwrap();
        });

        thread::spawn(move || {
            let val = String::from("hi from beta tx");
            beta_tx.send(val).unwrap();
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    #[test]
    fn sync_channel_demo() {
        let (tx, rx) = mpsc::sync_channel(0);
        // The capacity of the channel is 0, so the send operation will block until the receiver is ready to receive a value
        // Sync channel is useful when you want to avoid the out-of-memory error

        let handle = thread::spawn(move || {
            let val = String::from("hi");
            println!("Before sub thread sending");
            tx.send(val).unwrap();
            println!("After sub thread sending");
        });

        println!("Before main thread sleep");
        thread::sleep(Duration::from_secs(1));
        println!("After main thread sleep");

        println!("Before main thread receiving");
        let received = rx.recv().unwrap();
        println!("Got: {}", received);
        println!("After main thread receiving");
        handle.join().unwrap();
    }
}
