use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JourneyRequest {
    pub from: String,
    pub max_duration: i32,
    pub count: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StopAreaRequest {
    pub name: String,
}
