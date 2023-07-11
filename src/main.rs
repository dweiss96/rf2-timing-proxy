use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;

use tokio::sync::RwLock;
use tokio::task;
use warp::Filter;

use crate::models::*;

mod background_tasks;
mod endurable;
mod handler;
mod models;

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

    init_tasks(clients.clone(), servers.clone());

    api_server.await
}

fn init_tasks(clients: Clients, servers: Servers) {
    task::spawn(background_tasks::ask_servers_for_data(servers.clone()));
    task::spawn(background_tasks::send_data(
        clients.clone(),
        servers.clone(),
    ));
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
fn with_servers(servers: Servers) -> impl Filter<Extract = (Servers,), Error = Infallible> + Clone {
    warp::any().map(move || servers.clone())
}
