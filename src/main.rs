use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use btleplug::api::{CharPropFlags, Peripheral as _};
use btleplug::platform::Peripheral;
use eyre::Result;
use futures_util::StreamExt;
use prost::Message as ProstMessage;
use prost_types::Timestamp;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    sync::{broadcast, mpsc},
    time,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

mod handle_peripheral;

mod heartrate {
    tonic::include_proto!("heartrate");
}

use heartrate::heart_rate_server::{HeartRate, HeartRateServer};
use heartrate::{Beat, Empty, Frame};

/// Only devices whose name contains this string will be tried.
const PERIPHERAL_ADDR_MATCH: &str = "D0:0E:F7:6F:5F:88";
/// UUID of the characteristic for which we should subscribe to notifications.
const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x00002a37_0000_1000_8000_00805f9b34fb);

struct AppState {
    tx: broadcast::Sender<Frame>,
}

#[derive(Clone)]
struct MyHeartRate {
    tx: broadcast::Sender<Frame>,
}

#[tonic::async_trait]
impl HeartRate for MyHeartRate {
    type StreamRateStream = ReceiverStream<Result<Frame, Status>>;

    async fn stream_rate(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::StreamRateStream>, Status> {
        let mut rx = self.tx.subscribe();
        let (tx_mpsc, rx_mpsc) = mpsc::channel(128);

        tokio::spawn(async move {
            while let Ok(frame) = rx.recv().await {
                if tx_mpsc.send(Ok(frame)).await.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx_mpsc)))
    }
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let mut rx = state.tx.subscribe();
    while let Ok(frame) = rx.recv().await {
        let mut buf = Vec::new();
        if frame.encode(&mut buf).is_ok() {
            if socket.send(Message::Binary(buf.into())).await.is_err() {
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let (tx, _) = broadcast::channel(4);
    let app_state = Arc::new(AppState { tx: tx.clone() });

    // BLE Task
    let ble_tx = tx.clone();
    tokio::spawn(async move {
        if let Err(e) = ble_loop(ble_tx).await {
            eprintln!("BLE loop error: {:?}", e);
        }
    });

    let grpc_service = MyHeartRate { tx: tx.clone() };
    let grpc_addr = "[::1]:50051".parse()?;

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(app_state);

    let http_addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("HTTP/WebSocket server listening on {}", http_addr);
    println!("gRPC server listening on {}", grpc_addr);

    let axum_server = axum::serve(
        tokio::net::TcpListener::bind(http_addr).await.unwrap(),
        app.into_make_service(),
    );

    let grpc_server = Server::builder()
        .add_service(HeartRateServer::new(grpc_service))
        .serve(grpc_addr);

    tokio::select! {
        res = axum_server => {
            if let Err(e) = res {
                eprintln!("Axum server error: {:?}", e);
            }
        }
        res = grpc_server => {
            if let Err(e) = res {
                eprintln!("gRPC server error: {:?}", e);
            }
        }
    }

    Ok(())
}

async fn ble_loop(tx: broadcast::Sender<Frame>) -> Result<()> {
    loop {
        match handle_peripheral::get_peripherals(PERIPHERAL_ADDR_MATCH).await {
            Ok(peripheral) => {
                if let Err(e) = run_peripheral(peripheral, tx.clone()).await {
                    eprintln!("Peripheral error: {:?}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to find peripheral: {:?}. Starting simulation...", e);
                simulation_loop(tx.clone()).await?;
            }
        }
        time::sleep(Duration::from_secs(5)).await;
    }
}

async fn run_peripheral(peripheral: Peripheral, tx: broadcast::Sender<Frame>) -> Result<()> {
    if !peripheral.is_connected().await? {
        peripheral.connect().await?;
    }
    peripheral.discover_services().await?;

    for characteristic in peripheral.characteristics() {
        if characteristic.uuid == NOTIFY_CHARACTERISTIC_UUID
            && characteristic.properties.contains(CharPropFlags::NOTIFY)
        {
            peripheral.subscribe(&characteristic).await?;
            let mut notification_stream = peripheral.notifications().await?;
            while let Some(data) = notification_stream.next().await {
                if let Some(&val) = data.value.get(1) {
                    let beat = Beat {
                        value: val as i32,
                        ts: Some(Timestamp::from(std::time::SystemTime::now())),
                        device_id: peripheral.address().to_string(),
                    };
                    let frame = Frame {
                        payload: Some(heartrate::frame::Payload::Rate(beat)),
                    };
                    let _ = tx.send(frame);
                }
            }
        }
    }
    Ok(())
}

async fn simulation_loop(tx: broadcast::Sender<Frame>) -> Result<()> {
    println!("Starting simulation loop...");
    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        let val = 60 + rand::random::<u8>() % 40;
        let beat = Beat {
            value: val as i32,
            ts: Some(Timestamp::from(std::time::SystemTime::now())),
            device_id: "SIMULATED".to_string(),
        };
        let frame = Frame {
            payload: Some(heartrate::frame::Payload::Rate(beat)),
        };
        if tx.send(frame).is_err() {
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use heartrate::heart_rate_client::HeartRateClient;
    use tokio::sync::broadcast;

    #[tokio::test]
    async fn test_grpc_stream() -> Result<()> {
        let (tx, _) = broadcast::channel(128);
        let service = MyHeartRate { tx: tx.clone() };

        let addr: SocketAddr = "[::1]:50052".parse()?;
        let server_handle = tokio::spawn(async move {
            Server::builder()
                .add_service(HeartRateServer::new(service))
                .serve(addr)
                .await
                .unwrap();
        });

        // Give server time to start
        time::sleep(Duration::from_millis(200)).await;

        let mut client = HeartRateClient::connect("http://[::1]:50052").await?;
        let stream = client.stream_rate(Empty {}).await?.into_inner();
        let mut stream = stream.take(1);

        let beat = Beat {
            value: 75,
            ts: None,
            device_id: "TEST".to_string(),
        };
        let frame = Frame {
            payload: Some(heartrate::frame::Payload::Rate(beat)),
        };
        tx.send(frame)?;

        if let Some(res) = stream.next().await {
            let received_frame = res?;
            if let Some(heartrate::frame::Payload::Rate(received_beat)) = received_frame.payload {
                assert_eq!(received_beat.value, 75);
                assert_eq!(received_beat.device_id, "TEST");
            } else {
                panic!("Expected rate payload");
            }
        } else {
            panic!("Stream ended early");
        }

        server_handle.abort();
        Ok(())
    }

    #[tokio::test]
    async fn test_ws_handler() -> Result<()> {
        use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WsMessage};

        let (tx, _) = broadcast::channel(128);
        let app_state = Arc::new(AppState { tx: tx.clone() });
        let app = Router::new()
            .route("/ws", get(ws_handler))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;

        let server_handle = tokio::spawn(async move {
            axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        });

        let ws_url = format!("ws://{}/ws", addr);
        let (mut ws_stream, _) = connect_async(ws_url).await?;

        let beat = Beat {
            value: 80,
            ts: None,
            device_id: "WS_TEST".to_string(),
        };
        let frame = Frame {
            payload: Some(heartrate::frame::Payload::Rate(beat)),
        };
        tx.send(frame)?;

        if let Some(msg) = ws_stream.next().await {
            let msg = msg?;
            if let WsMessage::Binary(bin) = msg {
                let decoded_frame = Frame::decode(&bin[..])?;
                if let Some(heartrate::frame::Payload::Rate(received_beat)) = decoded_frame.payload
                {
                    assert_eq!(received_beat.value, 80);
                    assert_eq!(received_beat.device_id, "WS_TEST");
                } else {
                    panic!("Expected rate payload");
                }
            } else {
                panic!("Expected binary message, got {:?}", msg);
            }
        } else {
            panic!("WS stream ended early");
        }

        server_handle.abort();
        Ok(())
    }
}
