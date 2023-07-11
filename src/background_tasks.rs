use futures::{self, stream, StreamExt};
use serde_json;
use tokio::time;
use warp::ws::Message;

use crate::endurable::Endurable;
use crate::handler;
use crate::models::*;

pub async fn ask_servers_for_data(servers: Servers) {
    let interval = time::interval(1u8.seconds());

    let forever = stream::unfold(interval, |mut interval| async {
        interval.tick().await;
        for key in servers.read().await.keys() {
            match servers.read().await.get(key) {
                Some(s) => {
                    let session_info: Option<String> =
                        serde_json::to_string(&s.data.session.get("trackName")).ok();
                    let track_map_data: Option<String> =
                        serde_json::to_string(&s.data.trackmap.get("trackName")).ok();

                    match session_info {
                        Some(tn) => {
                            if tn != "" && tn != track_map_data.unwrap_or(String::new()) {
                                handler::trigger_trackmap(s).await;
                            }
                        }
                        None => {}
                    }
                }
                _ => {}
            }
        }

        Some(((), interval))
    });

    forever.for_each(|_| async {}).await;
}

pub async fn send_data(clients: Clients, servers: Servers) {
    let interval = time::interval(1u8.seconds());

    let forever = stream::unfold(interval, |mut interval| async {
        interval.tick().await;

        let svrs = servers.read().await;
        let mut data_str = "{".to_owned();
        for (sid, server) in svrs.clone() {
            let sname = server.server_id;
            let data = serde_json::to_string(&server.data).unwrap();
            data_str.push_str(
                format!("\"{}\":{{\"name\":\"{}\", \"data\":{}}},", sid, sname, data).as_str(),
            );
        }

        if data_str.len() > 1 {
            _ = data_str.pop();
        };
        data_str.push_str("}");

        clients.read().await.iter().for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::text(&data_str)));
            }
        });

        Some(((), interval))
    });

    forever.for_each(|_| async {}).await;
}
