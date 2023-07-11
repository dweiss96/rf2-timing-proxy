use std::collections::HashMap;
use std::sync::Arc;

use serde::Deserialize;
use serde_json;
use serde_json::Value;
use tokio::sync::RwLock;
use warp::Rejection;

pub use self::client::*;
pub use self::data::*;
pub use self::server::*;

mod client;
mod data;
mod server;

pub type Result<T> = std::result::Result<T, Rejection>;
//
// pub type ClientsMap = HashMap<String, Vec<Client>>;
// pub type ServerMap = HashMap<String, (Option<Server>, Data)>;

pub type Clients = Arc<RwLock<HashMap<String, Client>>>;
pub type Servers = Arc<RwLock<HashMap<String, Server>>>;

#[derive(Deserialize)]
pub struct DataRequest {
    pub topic: String,
    pub body: Value,
}
