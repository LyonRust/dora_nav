use dora_node_api::arrow::array::{AsArray, PrimitiveArray};
use dora_node_api::arrow::datatypes::UInt8Type;
use dora_node_api::{self, DoraNode, Event};
use eyre::ContextCompat;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use dora_node_api::arrow::array::{ArrayRef};
use dora_node_api::arrow::buffer::ScalarBuffer;
use std::str;

static LANGUAGE: &str = "Rust";
static PLATFORM: &str = "i7-8750@2.20GHz";
static NAME: &str = "dora-rs daemon Rust";

fn main() -> eyre::Result<()> {
    let (_node, mut events) = DoraNode::init_from_env()?;

    while let Some(event) = events.recv() {
        match event {
            Event::Input {
                id: _,
                data,
                metadata: _,
            } => {      
                // check if new size bracket
                let array = data
                    .as_any()
                    .downcast_ref::<PrimitiveArray<UInt8Type>>()
                    .expect("Expected a PrimitiveArray of UInt8Type");

                let buffer: &ScalarBuffer<u8> = array.values();

                let bytes: &[u8] = unsafe {
                    std::slice::from_raw_parts(buffer.as_ptr(), buffer.len())
                };
                
                match str::from_utf8(bytes) {
                    Ok(s) => println!("received message: {}", s),
                    Err(e) => println!("failed: {}", e),
                }
            
                // let time_u64 = array.get(0).context("could not slice data")?;
                // let t_send = uhlc::NTP64((*time_u64).into());

                // .to_vec() Data Latency
                // let _owned_data = array.to_vec();

                // Preallocated data
                // let _ = root_vec
                // .get_mut(&data.len())
                // .unwrap()
                // .copy_from_slice(array);

                // let t_received = system_time_clock();

                // latencies.push((t_received - t_send).to_duration());
                // let data_len = data.len() * 8;
                // if data_len != current_size {
                //     if n > 0 {
                //         record_results(start, current_size, n, latencies, latency, date);
                //     }
                //     current_size = data_len;
                //     n = 0;
                //     start = Instant::now();
                //     latencies = Vec::new();
                // }
                // n += 1;
            }
            Event::InputClosed { id } => {
                println!("Input `{id}` was closed");
            }
            other => eprintln!("Received unexpected input: {other:?}"),
        }
    }

    println!("finished");
    Ok(())
}

