#[cfg(test)]
mod tests {
    #[test]
    fn spwan_a_short_lived_thread() {
        fn find_max(arr: &[i32]) -> Option<i32> {
            const THRESHOLD: usize = 2;

            if arr.len() < THRESHOLD {
                return arr.iter().cloned().max();
            }

            let mid = arr.len() / 2;
            let (left, right) = arr.split_at(mid);

            crossbeam::scope(|s| {
                let thread_l = s.spawn(|_| find_max(left));
                let thread_r = s.spawn(|_| find_max(right));

                let max_l = thread_l.join().unwrap()?;
                let max_r = thread_r.join().unwrap()?;

                Some(max_l.max(max_r))
            })
            .unwrap()
        }

        let arr = &[1, 25, -4, 10];
        let max = find_max(arr);
        assert_eq!(max, Some(25));
    }

    #[test]
    fn create_a_parallel_pipeline() {
        use crossbeam::channel;
        use std::thread;
        use std::time::Duration;

        let (snd1, rcv1) = channel::bounded(1);
        let (snd2, rcv2) = channel::bounded(1);
        let n_msg = 10;
        let n_workers = 4;

        crossbeam::scope(|s| {
            s.spawn(|_| {
                for i in 0..n_msg {
                    snd1.send(i).unwrap();
                }
                drop(snd1);
            });

            for _ in 0..n_workers {
                let (sender, receiver) = (snd2.clone(), rcv1.clone());
                s.spawn(move |_| {
                    thread::sleep(Duration::from_millis(500));
                    for msg in receiver.iter() {
                        sender.send(msg * 2).unwrap();
                    }
                });
            }
            drop(snd2);

            let msgs = rcv2.iter().collect::<Vec<_>>();
            assert_eq!(msgs.len(), n_msg);
        })
        .unwrap();
    }

    #[test]
    fn pass_data_between_two_threads() {
        use crossbeam::channel;
        use std::{thread, time};

        let (snd, rcv) = channel::unbounded();
        let n_msgs = 10;
        crossbeam::scope(|s| {
            s.spawn(|_| {
                for i in 0..n_msgs {
                    snd.send(i).unwrap();
                    thread::sleep(time::Duration::from_millis(100));
                }
            });
        })
        .unwrap();
        for i in 0..n_msgs {
            let msg = rcv.recv().unwrap();
            assert_eq!(msg, i);
        }
    }
}
