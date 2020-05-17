mod tests{
    use num_cpus; // 1.13.0
    use std::time;
    use threadpool::ThreadPool; // 1.8.0 // 0.7.2
    use crossbeam; // 0.7.3
    use futures::executor; // 0.3.4
    use futures::Future;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use tokio::time::delay_for;
    use tokio::runtime::Runtime;

    #[test]
    fn s_1_3_sync_threads_with_crossbeam_messages() {
        let pool = ThreadPool::new(num_cpus::get());
        let (tx, rx) = crossbeam::unbounded();

        let tx_c = tx.clone();

        pool.execute(move || {
            println!("[Thread a] Spawned thread");

            time::Duration::from_millis(2000);
            tx_c.send("continue").unwrap();

            println!("[Thread a] End");
        });

        pool.execute(move || {
            println!("[Thread b] Spawned thread");
            while let Ok(message) = rx.recv() {
                println!("[Thread b] Message : {:?}", message);

                if message == "continue" {
                     println!("[Thread b] continue exectution {:?}", message);
                    break;
                }

                println!("[Thread b] End");
            }
        });

        pool.join();
    }

    #[test]
    fn s_1_3_sync_futures_with_crossbeam_messages_driver () {
        async fn sync_futures_with_crossbeam_messages() {
            let (tx, rx) = crossbeam::unbounded();

            let future_1 = async {
                println!("[Future 1] started");

                time::Duration::from_millis(2000);
                tx.send(1).unwrap();

                println!("[Future 1] End");
            };

            future_1.await;

            let future_2 = async {
                println!("[Future 2] started");
                time::Duration::from_millis(2000);

                while let Ok(message) = rx.recv() {
                    println!("[Future 2] Message : {:?}", message);

                    if message == 1 {
                        println!("[Future 2] continue exectution -> {:?}", message);
                        break;
                    }

                    println!("[Future 2] End");
                }
            };

            future_2.await;
        }

        let fut = async {
            sync_futures_with_crossbeam_messages().await;
        };

       executor::block_on(fut);
    }

    #[test]
    fn s_1_5_1_concurrent_updates_futures_to_arc_mutex() {
    async fn concurrent_updates_futures_to_arc_mutex() {
        let s = Arc::new(Mutex::new(1_u128));

        let i = s.clone();
        let j = s.clone();

        let future_1 = tokio::spawn(async move {
            println!("[Future 1] started");

            for _item in 0..30 {
                tokio::time::delay_for(Duration::from_secs(1)).await;

                {
                    let mut i_shared = i.lock().unwrap();
                    println!("[Future 1] {:?}", i_shared);
                    *i_shared = *i_shared * 3;
                }
            }

            println!("[Future 1] End -> i = {:?}", i);
        });


        let future_2 = tokio::spawn(async move {
            println!("[Future 2] started");

            for _item in 0..30 {
                tokio::time::delay_for(Duration::from_secs(1)).await;

                {
                    let mut i_shared = j.lock().unwrap();
                    println!("[Future 2] {:?}", i_shared);
                    *i_shared = *i_shared * 2;
                }
            }

            println!("[Future 2] End -> i = {:?}", j);

        });

        tokio::join!(future_1, future_2);

        println!("Final i = {:?}", s);
    }


    let fut = async {
        concurrent_updates_futures_to_arc_mutex().await;
    };

    let mut rt = Runtime::new().unwrap();

    rt.block_on(fut);

}
}

use num_cpus; // 1.13.0
use std::time;
use threadpool::ThreadPool; // 1.8.0 // 0.7.2

fn main() {

}
