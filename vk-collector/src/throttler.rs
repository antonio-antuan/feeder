use async_trait::async_trait;
use futures::stream::FuturesUnordered;
use std::{collections::VecDeque, sync::Arc};
use tokio::{sync::Mutex, task::JoinHandle, time};
use tokio_stream::StreamExt;

#[async_trait]
pub trait Worker<T> {
    async fn call(&self, job: T);
}

pub struct Throttler<T, W>
where
    W: Worker<T> + Clone + Send + 'static,
    T: Send,
{
    tick_interval: time::Duration,
    delayed: Arc<Mutex<VecDeque<T>>>,
    worker: W,
}

impl<T, W> Throttler<T, W>
where
    W: Worker<T> + Clone + Send + 'static,
    T: Send + 'static,
{
    pub fn new(tick_interval: time::Duration, worker: W) -> Self {
        Self {
            tick_interval,
            worker,
            delayed: Default::default(),
        }
    }

    pub async fn push(&mut self, job: T) {
        self.delayed.lock().await.push_back(job)
    }

    pub fn run(&self, batch_size_per_tick: usize) -> JoinHandle<()> {
        let mut ticker = time::interval(self.tick_interval);
        let delayed = self.delayed.clone();
        let worker = self.worker.clone();
        tokio::spawn(async move {
            let mut fo = FuturesUnordered::new();
            loop {
                tokio::select! {
                    t = ticker.tick() => {
                        while let Some(_) = fo.next().await {}
                        log::trace!("new tick: {:?}", t);
                        let mut to_delay = batch_size_per_tick - fo.len();
                        if to_delay <= 0 {
                            log::trace!("no space in fo");
                            continue
                        } else {
                            let mut guard = delayed.lock().await;
                            if guard.is_empty() {
                                log::trace!("no delayed tasks");
                                continue
                            } else if guard.len() < to_delay {
                                to_delay = guard.len();
                            }
                            log::trace!("delay {} new tasks", to_delay);
                            fo.extend(guard.drain(0..to_delay).map(|j|worker.call(j)));
                        }
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::throttler::{Throttler, Worker};
    use async_trait::async_trait;
    use std::sync::Arc;
    use tokio::{
        sync::{mpsc, Mutex},
        time,
    };

    struct Foo {
        res: Mutex<mpsc::Sender<i32>>,
    }

    impl Foo {
        pub fn new(res: mpsc::Sender<i32>) -> Self {
            Self {
                res: Mutex::new(res),
            }
        }
    }

    #[derive(Debug)]
    struct Job {
        i: i32,
    }

    #[async_trait]
    impl Worker<Job> for Arc<Foo> {
        async fn call(&self, job: Job) {
            self.res.lock().await.send(job.i).await;
        }
    }

    #[tokio::test]
    async fn test_throttler() {
        let (s, mut r) = mpsc::channel(1);
        let mut t = Throttler::new(time::Duration::from_millis(30), Arc::new(Foo::new(s)));
        t.push(Job { i: 1 }).await;

        let h = t.run(1);
        let res = time::timeout(time::Duration::from_millis(10), r.recv()).await;
        assert!(res.is_err());

        let res = time::timeout(time::Duration::from_millis(25), r.recv()).await;
        assert_eq!(res, Ok(Some(1)));

        let res = time::timeout(time::Duration::from_millis(70), r.recv()).await;
        assert!(res.is_err());

        t.push(Job { i: 99 }).await;
        let res = time::timeout(time::Duration::from_millis(30), r.recv()).await;
        assert_eq!(res, Ok(Some(99)));
    }
}
