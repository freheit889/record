//有bug  没搞明白怎么写
use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    let child=thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut status_share=status_shared.lock().unwrap();
            status_share.jobs_completed += 1;
        }
    }).join();
    
    while status_shared.jobs_completed< 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
