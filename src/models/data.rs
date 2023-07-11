use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Deserialize)]
pub struct EssentialTrackMapData {
    pub track_name: String,
}
#[derive(Deserialize)]
pub struct EssentialSessionInfo {
    pub track_name: String,
}
#[derive(Clone, Serialize)]
pub struct Data {
    pub standings: Vec<Value>,
    pub session: Map<String, Value>,
    pub trackmap: Map<String, Value>,
}
