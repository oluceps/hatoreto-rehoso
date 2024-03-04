use btleplug::api::{CharPropFlags, Peripheral};
use eyre::Result;
use futures::stream::StreamExt;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    sync::mpsc::{self, channel, Receiver, Sender},
    time,
};
use uuid::Uuid;

mod handle_peripheral;
mod heartrate;

use heartrate::heart_server;
mod srv;

/// Only devices whose name contains this string will be tried.
const PERIPHERAL_ADDR_MATCH: &str = "D0:0E:F7:6F:5F:88";
/// UUID of the characteristic for which we should subscribe to notifications.
const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x00002a37_0000_1000_8000_00805f9b34fb);

pub use heartrate::Rate;

use crate::{heartrate::heart_server::HeartServer, srv::HeartRate};

#[tokio::main]
async fn main() -> Result<()> {
    use tonic::transport::Server;
    use tonic::Status;

    pretty_env_logger::init();

    tonic_build::compile_protos("../rate.proto")?;

    let addr = "[::1]:7000".parse()?;

    let (tx, rx): (
        mpsc::Sender<Result<Rate, Status>>,
        mpsc::Receiver<Result<Rate, Status>>,
    ) = mpsc::channel(8);

    let heartrate_service: HeartServer<HeartRate> =
        HeartServer::from_arc(Arc::new(HeartRate::from_rx(Arc::new(rx))));

    tokio::spawn(async move {
        let _ = Server::builder()
            .add_service(heartrate_service)
            .serve(addr)
            .await;
    });

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
                let mut notification_stream = peripheral.notifications().await?;
                while let Some(data) = notification_stream.next().await {
                    let msg = data.value.get(1).unwrap();
                    let rate = Rate { value: *msg as i32 };
                    println!("Received data  {:?}", msg);
                    match tx.send(Result::<_, Status>::Ok(rate)).await {
                        Ok(_) => (),
                        Err(_) => (),
                    }
                }
            }
        }
        println!("Disconnecting from peripheral {:?}...", local_addr);
        peripheral.disconnect().await?;
    }
    Ok(())
}
