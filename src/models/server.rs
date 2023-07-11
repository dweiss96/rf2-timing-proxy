use std::time::Instant;

use futures::{self, FutureExt, StreamExt};
use serde_json::{self, from_str};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

use crate::models::data::Data;
use crate::models::{DataRequest, Servers};

#[derive(Clone)]
pub struct Server {
    pub server_id: String,
    pub latest_message: Instant,
    pub data: Data,
    pub sender: Option<mpsc::UnboundedSender<Result<Message, warp::Error>>>,
}
impl Server {
    pub async fn connect(ws: WebSocket, id: String, servers: Servers, mut server: Server) {
        let (ws_sender, mut ws_rcv) = ws.split();
        let (sender, receiver) = mpsc::unbounded_channel();

        let receiver = UnboundedReceiverStream::new(receiver);
        tokio::task::spawn(receiver.forward(ws_sender).map(|result| {
            if let Err(e) = result {
                eprintln!("error sending websocket msg: {}", e);
            }
        }));

        server.sender = Some(sender);
        servers.write().await.insert(id.clone(), server);

        while let Some(result) = ws_rcv.next().await {
            let msg = match result {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("error receiving ws message for id: {}): {}", id.clone(), e);
                    break;
                }
            };
            Server::server_msg(&id, msg, &servers).await;
        }

        servers.write().await.remove(&id);
    }

    async fn server_msg(id: &str, msg: Message, servers: &Servers) {
        let message = match msg.to_str() {
            Ok(v) => v,
            Err(_) => return,
        };
        let data_req: DataRequest = match from_str(&message) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("error while parsing message to data request: {}", e);
                return;
            }
        };

        let server_item = servers.read().await.get(id).cloned();
        match server_item {
            Some(mut i) => {
                let mut server_data = i.data;
                match data_req.topic.as_str() {
                    "LiveStandings" => {
                        server_data.standings = data_req.body.as_array().unwrap().to_owned()
                    }
                    "TrackMap" => {
                        server_data.trackmap = data_req.body.as_object().unwrap().to_owned()
                    }
                    "SessionInfo" => {
                        server_data.session = data_req.body.as_object().unwrap().to_owned()
                    }
                    _ => return,
                };
                i.latest_message = Instant::now();
                i.data = server_data;
                servers.write().await.insert(id.to_string(), i);
            }
            None => return,
        };
    }
}
