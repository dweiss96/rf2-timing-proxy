use futures;
use futures::{stream, StreamExt}; // 0.3.13
use serde::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, RwLock};
use tokio::{task, time}; // 1.3.0
use warp::{ws::Message, Filter, Rejection};

mod handler;
mod rf2ws;
mod ws;

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<RwLock<HashMap<String, Client>>>;
type Servers = Arc<RwLock<HashMap<String, Server>>>;


#[derive(Deserialize)]
pub struct EssentialTrackMapData {
  pub track_name: std::string::String
}
#[derive(Deserialize)]
pub struct EssentialSessionInfo {
  pub track_name: std::string::String
}
#[derive(Clone, Serialize)]
pub struct Data {
    pub standings: Vec<serde_json::Value>,
    pub session: serde_json::Map<String, serde_json::Value>,
    pub trackmap: serde_json::Map<String, serde_json::Value>,
}
#[derive(Clone)]
pub struct Client {
    pub latest_ping: Instant,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}
#[derive(Clone)]
pub struct Server {
    pub server_id: std::string::String,
    pub latest_message: Instant,
    pub data: Data,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));
    let servers: Servers = Arc::new(RwLock::new(HashMap::new()));

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let stats = warp::path!("stats")
        .and(with_clients(clients.clone()))
        .and(with_servers(servers.clone()))
        .and_then(handler::stat_handler);

    let read_ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(handler::read_ws_handler);

    let write_ws_route = warp::path("rf2ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_servers(servers.clone()))
        .and_then(handler::write_ws_handler);

    let routes = health_route
        .or(read_ws_route)
        .or(write_ws_route)
        .or(stats)
        .with(warp::cors().allow_any_origin());

    let api_server = warp::serve(routes).run(([0, 0, 0, 0], 5398));

    // send_data.await;
    // api_server.await

    //let results = (send_data(&clients, &servers).await, api_server.await);

    init_tasks(clients.clone(), servers.clone());

    // task::spawn(ask_servers_for_data(&servers));

    // task::spawn(async {
    //   send_data(&clients, &servers).await;
    // });
    api_server.await
    // (
    //   ask_servers_for_data(&servers).await,
    //   clear_clients(&clients).await,
    //   clear_servers(&servers).await,
    //   api_server.await
    // );
}

fn init_tasks(clients: Clients, servers: Servers) {
    task::spawn(ask_servers_for_data(servers.clone()));
    task::spawn(send_data(clients.clone(), servers.clone()));
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
fn with_servers(servers: Servers) -> impl Filter<Extract = (Servers,), Error = Infallible> + Clone {
    warp::any().map(move || servers.clone())
}

async fn ask_servers_for_data(servers: Servers) {
    let interval = time::interval(Duration::from_secs(1));

    let forever = stream::unfold(interval, |mut interval| async {
        interval.tick().await;
        for key in servers.read().await.keys() {
            match servers.read().await.get(key) {
              Some(s) => {
                let session_info: Option<String> = serde_json::to_string(&s.data.session.get("trackName")).ok();
                let track_map_data: Option<String> = serde_json::to_string(&s.data.trackmap.get("trackName")).ok();

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

async fn send_data(clients: Clients, servers: Servers) {
    let interval = time::interval(Duration::from_secs(1));

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
