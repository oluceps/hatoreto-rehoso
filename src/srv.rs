use tonic::{transport::Server, Request, Response, Status};

use std::sync::Arc;

type RateRx = Arc<mpsc::Receiver<Result<Rate, Status>>>;

pub struct HeartRate {
    rate_rx: RateRx,
}

impl HeartRate {
    pub fn from_rx(rate_rx: RateRx) -> Self {
        Self { rate_rx }
    }
}

pub mod heartrate {
    tonic::include_proto!("heartrate");
}

use crate::channel;
use crate::heartrate::heart_server::{Heart, HeartServer};
use crate::heartrate::{heart_server, Rate};
use futures_util::Stream;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::time::Duration;

type ResponseStream = Pin<Box<dyn Stream<Item = Result<Rate, Status>> + Send>>;

#[tonic::async_trait]
impl Heart for HeartRate {
    type BeatStream = ResponseStream;

    async fn beat(
        &self,
        request: tonic::Request<tonic::Streaming<()>>,
    ) -> std::result::Result<tonic::Response<Self::BeatStream>, tonic::Status> {
        use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};

        println!("Got a request: {:?}", request);

        let rx = self.rate_rx.clone();
        let rx = Arc::try_unwrap(rx).unwrap();

        let output_stream = ReceiverStream::new(rx);

        Ok(Response::new(Box::pin(output_stream)))
    }
}
