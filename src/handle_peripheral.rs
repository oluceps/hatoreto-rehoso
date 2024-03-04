use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Manager, Peripheral};
use eyre::{eyre, Result};
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

pub async fn get_peripherals(addr: &str, char_uuid: Uuid) -> Result<Peripheral> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
    }

    for adapter in adapter_list.iter() {
        println!("Starting scan...");
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(2)).await;
        let peripherals = adapter.peripherals().await?;

        if peripherals.is_empty() {
            eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
        } else {
            // All peripheral devices in range.
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;

                let local_addr = properties.unwrap().address.to_string();
                dbg!(local_addr.clone());

                if local_addr == addr {
                    return Ok(peripheral.clone());
                }
            }
        }
    }
    Err(eyre!("not found"))
}
