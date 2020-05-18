mod tests {
    use tokio::runtime::Runtime;
    use crossbeam_channel::unbounded;

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
