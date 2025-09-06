//! Handles the communication loop with lqosd.

use crate::bus::cpu_ram::{RAM_USED, SHOULD_EXIT, TOTAL_RAM};
use anyhow::Result;
use lqos_bus::{LibreqosBusClient, BusRequest, BusResponse};
use tokio::sync::mpsc::Receiver;
pub mod cpu_ram;
use std::{
    io::stdout,
    sync::atomic::{AtomicBool, Ordering},
};


/// The main loop for the bus.
/// Spawns a separate task to handle the bus communication.
pub async fn bus_loop() {
    tokio::spawn(cpu_ram::gather_sysinfo());
    main_loop_wrapper().await;
}

async fn main_loop_wrapper() {
    let loop_result = main_loop().await;
    if let Err(e) = loop_result {
        SHOULD_EXIT.store(true, Ordering::Relaxed);
        panic!("Error in main loop: {}", e);
    }
}

async fn main_loop() -> Result<()> {

    let mut bus_client = LibreqosBusClient::new().await?;

    println!("hi main_loop");

    loop {

        // Perform actual bus collection
        let mut commands: Vec<BusRequest> = Vec::new();

        commands.push(BusRequest::GetCurrentThroughput);

        // Send the requests and process replies
        for response in bus_client.request(commands).await? {
            match response {
                BusResponse::CurrentThroughput { .. } => {
                    println!("CurrentThroughput response: {:?}",response)
                }
                _ => {}
            }
        }

        // // Check if we should be quitting
        if SHOULD_EXIT.load(Ordering::Relaxed) {
            break;
        }

        println!("hello again {:?} / {:?}",RAM_USED,TOTAL_RAM);

        // Sleep for one tick
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    }
    println!("bye main_loop");

    Ok(())
}
