mod tests {
    use tokio::runtime::Runtime;
    use crossbeam_channel::unbounded;
    use tokio::sync::Semaphore;
    use std::sync::{Arc};

    #[test]
    fn s_3_5_multiplex_with_limit () {
        async fn multiplex_with_limit () {
            let multiplex_limit = Arc::new(Semaphore::new(10));
            let critical_section = Arc::new(Semaphore::new(1));
            let mut i: u32 = 0;
            let mut j: u32 = 0;
            let mut k: u32 = 0;

            for worker in 0..30 {
                let multiplex_limit_clone = multiplex_limit.clone();
                let critical_section_clone = critical_section.clone();

                let _new_client = tokio::spawn(async move {
                    let p1 = multiplex_limit_clone.try_acquire();

                    if p1.is_ok() {
                        let _p2 = critical_section_clone.acquire().await;

                        // do some secret stuff.
                        i += 1;
                        j += 1;
                        k += 1;

                        println!("Doing secret stuff .. {:?}", i);

                    } else {
                        println!("Thread limit over 10 reached., {:?}", i);
                    }
                });
            }
        }

        let fut = async {
            multiplex_with_limit().await;
        };

        let mut rt = Runtime::new().unwrap();

        rt.block_on(fut);
    }

    #[test]
    fn s_3_3_1_crossbeam_rendezvous_signal_pattern() {
        async fn crossbeam_rendezvous_signal_pattern() {
            let (tx, rx) = unbounded();
            let (rev_tx, rev_rx) = unbounded();

            let future_1 = tokio::spawn(async move {
                println!("[Future 1] started");

                tx.send(1).unwrap();

                if rev_rx.recv().unwrap() == 2 {
                    println!("[Future 1] Receied value 2");
                    tx.send(3).unwrap();
                }

                println!("[Future 1] End");
            });


            let future_2 = tokio::spawn(async move {
                println!("[Future 2] started");

                if rx.recv().unwrap() == 1 {
                    println!("[Future 2] Received 1 from future 1");
                    rev_tx.send(2).unwrap();
                }

                if rx.recv().unwrap() == 3 {
                    println!("[Future 2] Received 3 from future 1");

                    rev_tx.send(4).unwrap();
                }

                println!("[Future 2] End");
            });

            let _res = tokio::join!(future_1, future_2);

            println!("Final");
        }


        let fut = async {
            crossbeam_rendezvous_signal_pattern().await;
        };

        let mut rt = Runtime::new().unwrap();

        rt.block_on(fut);
    }
}

fn main() {
}
