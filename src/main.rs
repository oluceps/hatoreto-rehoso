use btleplug::api::{CharPropFlags, Peripheral};
use eyre::Result;
use futures::stream::StreamExt;
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
    time,
};
use uuid::Uuid;

mod handle_peripheral;

/// Only devices whose name contains this string will be tried.
const PERIPHERAL_ADDR_MATCH: &str = "D0:0E:F7:6F:5F:88";
/// UUID of the characteristic for which we should subscribe to notifications.
const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x00002a37_0000_1000_8000_00805f9b34fb);

struct Rate<T>(Arc<Mutex<(Sender<T>, Receiver<T>)>>);

impl Rate<T> {
    fn new() -> Self {
        Arc::new(Mutex::new(channel(1)))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let rate_chan: Rate<u16> = Rate::new();

    let peripheral =
        handle_peripheral::get_peripherals(PERIPHERAL_ADDR_MATCH, NOTIFY_CHARACTERISTIC_UUID)
            .await?;

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
                // Print the first 4 notifications received.
                let mut notification_stream = peripheral.notifications().await?;
                // Process while the BLE connection is not broken or stopped.
                while let Some(data) = notification_stream.next().await {
                    println!("Received data  {:?}", data.value.get(1).unwrap());
                }
            }
        }
        println!("Disconnecting from peripheral {:?}...", local_addr);
        peripheral.disconnect().await?;
    }
    Ok(())
}
