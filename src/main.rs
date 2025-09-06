mod bus;
use std::{sync::atomic::Ordering, thread};

use anyhow::Result;

use crate::bus::cpu_ram::SHOULD_EXIT;

fn main() -> Result<()> {

    println!("hi");

    // // Create a tokio runtime in a single thread
    // std::thread::spawn(move || {
    //     let rt = tokio::runtime::Runtime::new().unwrap();
    //     rt.block_on(async move { bus::bus_loop().await });
    // });

    ctrlc::set_handler(move || {
        println!("ctl-c!");
        SHOULD_EXIT.store(true, Ordering::Relaxed);
        println!("waiting 5 secs to exit the loops");
        thread::sleep(std::time::Duration::from_secs(5));
        std::process::exit(0);
    })
    .unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move { bus::bus_loop().await });

    println!("OK bye");

    // Return OK
    Ok(())
}
