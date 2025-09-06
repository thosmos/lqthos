//! Handles the communication loop with lqosd.

use crate::bus::cpu_ram::{RAM_USED, SHOULD_EXIT, TOTAL_RAM};
use anyhow::Result;
use lqos_bus::{BusRequest, BusResponse, LibreqosBusClient, StatsRequest};
pub mod cpu_ram;
use std::{net::{IpAddr, Ipv4Addr}, str::FromStr, sync::atomic::Ordering};


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

        // // Check if we should be quitting
        if SHOULD_EXIT.load(Ordering::Relaxed) {
            break;
        }

        // Perform actual bus collection
        let mut commands: Vec<BusRequest> = Vec::new();

        // commands.push(BusRequest::GetCurrentThroughput);
        // commands.push(BusRequest::GetHostCounter);
        // commands.push(BusRequest::GetLongTermStats(StatsRequest::AllHosts));
        commands.push(BusRequest::GetAllCircuits);

        // Send the requests and process replies
        for response in bus_client.request(commands).await? {
            match response {
                BusResponse::CurrentThroughput { .. } => {
                    println!("CurrentThroughput response: {:?}",response)
                }
                BusResponse::HostCounters { .. } => {
                    println!("Host counters: {:?}",response)
                }
                BusResponse::LongTermHosts { .. } => {
                    println!("Long Term Hosts: {:?}",response)
                }
                BusResponse::CircuitData { .. } => {
                    // println!("Circuits: {:?}", response);
                    if let BusResponse::CircuitData(circuits) = response {
                        for circuit in circuits {
                            if circuit.ip == Ipv4Addr::new(10,38,0,14) {
                                println!("circuit: {:?}",circuit);
                            }
                        }
                    }
                }
                _ => {}
            }
        }


        // println!("hello again {:?} / {:?}",RAM_USED,TOTAL_RAM);

        // // Check if we should be quitting
        if SHOULD_EXIT.load(Ordering::Relaxed) {
            break;
        }

        // Sleep for one tick
        tokio::time::sleep(std::time::Duration::from_millis(3000)).await;


    }
    println!("bye main_loop");

    Ok(())
}
