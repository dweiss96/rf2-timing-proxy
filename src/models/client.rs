use std::time::Instant;

use futures::{self, FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

use crate::models::Clients;

#[derive(Clone)]
pub struct Client {
    pub latest_ping: Instant,
    pub sender: Option<mpsc::UnboundedSender<Result<Message, warp::Error>>>,
}
impl Client {
    pub async fn connect(ws: WebSocket, id: String, clients: Clients, mut client: Client) {
        let (client_ws_sender, mut client_ws_rcv) = ws.split();
        let (client_sender, client_rcv) = mpsc::unbounded_channel();

        let client_rcv = UnboundedReceiverStream::new(client_rcv);
        tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
            if let Err(e) = result {
                eprintln!("error sending websocket msg: {}", e);
            }
        }));

        client.sender = Some(client_sender);
        clients.write().await.insert(id.clone(), client);

        while let Some(result) = client_ws_rcv.next().await {
            match result {
                Ok(_msg) => {}
                Err(e) => {
                    eprintln!("error receiving ws message for id: {}): {}", id.clone(), e);
                    break;
                }
            };
        }

        clients.write().await.remove(&id);
    }
}
