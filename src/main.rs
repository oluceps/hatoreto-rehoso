use btleplug::api::{CharPropFlags, Peripheral};
use eyre::Result;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    time,
};
use tokio_stream::StreamExt;
use tonic::{transport::Server, Status};
use uuid::Uuid;

use crate::heartrate::Beat;

// use crate::heartrate::HeartRateData;

mod handle_peripheral;

/// Only devices whose name contains this string will be tried.
const PERIPHERAL_ADDR_MATCH: &str = "D0:0E:F7:6F:5F:88";
/// UUID of the characteristic for which we should subscribe to notifications.
const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x00002a37_0000_1000_8000_00805f9b34fb);

mod heartrate {
    tonic::include_proto!("heartrate");
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    // let addr = "[::1]:7000".parse()?;

    type RateRes = Result<Beat, Status>;

    let (tx, rx): (Sender<RateRes>, Receiver<RateRes>) = channel(8);

    let beat = Beat::default();

    let peripheral = handle_peripheral::get_peripherals(PERIPHERAL_ADDR_MATCH).await?;

    let properties = peripheral.properties().await?.unwrap();
    let local_addr = properties.address.to_string();
    let is_connected = peripheral.is_connected().await?;

    println!(
        "Peripheral {:?} is connected: {:?}",
        &local_addr, is_connected
    );

    // Check if it's the peripheral we want.
    println!("Found matching peripheral {:?}...", &local_addr);
    if !is_connected {
        // Connect if we aren't already connected.
        if let Err(err) = peripheral.connect().await {
            eprintln!("Error connecting to peripheral, skipping: {}", err);
        }
    }
    let is_connected = peripheral.is_connected().await?;
    println!(
        "Now connected ({:?}) to peripheral {:?}.",
        is_connected, &local_addr
    );
    if is_connected {
        println!("Discover peripheral {:?} services...", local_addr);
        peripheral.discover_services().await?;

        for characteristic in peripheral.characteristics() {
            // Subscribe to notifications from the characteristic with the selected
            // UUID.
            if characteristic.uuid == NOTIFY_CHARACTERISTIC_UUID
                && characteristic.properties.contains(CharPropFlags::NOTIFY)
            {
                println!("Subscribing to characteristic {:?}", characteristic.uuid);
                peripheral.subscribe(&characteristic).await?;
                let mut notification_stream = peripheral.notifications().await?;
                while let Some(data) = notification_stream.next().await {
                    // let msg = data.value.get(1).unwrap();
                    // let rate = Rate { value: *msg as i32 };
                    // TODO: https://www.bluetooth.com/specifications/specs/gatt-specification-supplement-5/ 0x2A37 ?
                    println!("Received data  {:?}", data.value.get(1));
                    // match tx.send(Result::<_, Status>::Ok(msg)).await {
                    //     Ok(_) => (), // TODO: what?
                    //     Err(_) => (),
                    // }
                }
            }
        }
        println!("Disconnecting from peripheral {:?}...", local_addr);
        peripheral.disconnect().await?;
    }
    Ok(())
}
