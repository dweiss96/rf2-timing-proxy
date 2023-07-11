use std::time::Instant;

use uuid::Uuid;
use warp::{http::StatusCode, ws::Message, Reply};

use crate::models::*;

pub async fn trigger_trackmap(server: &Server) {
    if let Some(sender) = &server.sender {
        let _ = sender.send(Ok(Message::text("track_map")));
    }
}

pub async fn stat_handler(clients: Clients, servers: Servers) -> Result<impl Reply> {
    let num_of_clients = clients.read().await.iter().count();
    let num_of_servers = servers.read().await.iter().count();

    Ok(format!(
        "{{\"serverCount\": {}, \"clientCounts\": {}}}",
        num_of_servers, num_of_clients
    ))
}

pub async fn read_ws_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply> {
    let uuid = Uuid::new_v4().as_simple().to_string();

    clients.write().await.insert(
        uuid.clone(),
        Client {
            latest_ping: Instant::now(),
            sender: None,
        },
    );
    let client = clients.read().await.get(uuid.clone().as_str()).cloned();
    match client {
        Some(c) => {
            Ok(ws.on_upgrade(move |socket| Client::connect(socket, uuid.clone(), clients, c)))
        }
        None => Err(warp::reject::not_found()),
    }
}

pub async fn write_ws_handler(
    ws: warp::ws::Ws,
    name: std::string::String,
    servers: Servers,
) -> Result<impl Reply> {
    let uuid = Uuid::new_v4().as_simple().to_string();

    servers.write().await.insert(
        uuid.clone(),
        Server {
            server_id: name,
            data: Data {
                standings: Vec::new(),
                session: serde_json::Map::new(),
                trackmap: serde_json::Map::new(),
            },
            latest_message: Instant::now(),
            sender: None,
        },
    );
    let server = servers.read().await.get(uuid.clone().as_str()).cloned();
    match server {
        Some(s) => {
            Ok(ws.on_upgrade(move |socket| Server::connect(socket, uuid.clone(), servers, s)))
        }
        None => Err(warp::reject::not_found()),
    }
}

pub async fn health_handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}
