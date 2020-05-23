use crate::errors::Result as RfResult;
use rusqlite::Connection;
use std::path::Path;

type DbJob<T> = Box<dyn FnOnce(&mut Option<T>) -> RfResult<()> + Send>;
pub type Conn = Connection;

#[derive(Clone)]
pub struct DbQueryExecutor {
    inner: std::sync::mpsc::Sender<DbJob<Conn>>,
}

impl DbQueryExecutor {
    pub fn spawn<F: FnOnce(&mut Option<Conn>) -> RfResult<()> + Send + 'static>(&self, job: F) {
        self.inner
            .send(Box::new(job))
            .expect("thread_pool::Executor::spawn failed");
    }
}

pub fn start_thread(db_path: &Path) -> (std::thread::JoinHandle<()>, DbQueryExecutor) {
    let (sender, receiver) = std::sync::mpsc::channel::<DbJob<Conn>>();
    let db_path: std::path::PathBuf = db_path.into();
    let join_handle = std::thread::spawn(move || {
        let mut db = match Conn::open(db_path) {
            Ok(db) => {
                Some(db)
            }
            Err(err) => {
                println!("Initialiazion cause error: {}", err);
                None
            }
        };
        loop {
            match receiver.recv() {
                Ok(x) => {
                    let r = x(&mut db);
                    match r {
                        Ok(_) => (),
                        Err(e) => println!("{}", e.to_string()),
                    }
                }
                Err(err) => {
                    println!("db_thread: recv error: {}", err);
                    break;
                }
            }
        }
    });
    (join_handle, DbQueryExecutor { inner: sender })
}
