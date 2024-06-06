use std::{thread, time::Duration};

use tokio::{
    fs,
    runtime::{Builder, Runtime},
    time::sleep,
};

fn main() {
    let handle = thread::spawn(|| {
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        rt.spawn(async {
            println!("Hello, world11111!");
        });

        rt.block_on(run(&rt));
    });

    handle.join().unwrap();
}

fn expensive_blocking_task(s: String) -> String {
    thread::sleep(Duration::from_millis(1000));
    blake3::hash(s.as_bytes()).to_string()
}

async fn run(rt: &Runtime) {
    rt.spawn(async {
        println!("future 1");
        let content = fs::read("Cargo.toml").await.unwrap();
        println!("content: {:?}", content.len());
    });
    rt.spawn(async {
        println!("future 2");
        let result = expensive_blocking_task("hello".to_string());
        println!("result: {}", result);
    });
    sleep(Duration::from_millis(2000)).await;
}
