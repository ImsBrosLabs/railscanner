use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopAreasResponse {
    pub places: Vec<Place>,
    pub warnings: Vec<Warning>,
    #[serde(rename = "feed_publishers")]
    pub feed_publishers: Vec<FeedPublisher>,
    pub context: Context,
    pub links: Vec<Link>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    pub id: String,
    pub name: String,
    pub quality: i64,
    #[serde(rename = "embedded_type")]
    pub embedded_type: String,
    #[serde(rename = "administrative_region")]
    pub administrative_region: Option<AdministrativeRegion>,
    #[serde(rename = "stop_area")]
    pub stop_area: Option<StopArea>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdministrativeRegion {
    pub id: String,
    pub name: String,
    pub label: String,
    #[serde(rename = "zip_code")]
    pub zip_code: String,
    pub coord: Coord,
    pub insee: String,
    pub level: i64,
    #[serde(rename = "administrative_regions")]
    pub administrative_regions: Vec<AdministrativeRegion2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coord {
    pub lat: String,
    pub lon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdministrativeRegion2 {
    pub id: String,
    pub insee: String,
    pub name: String,
    pub label: String,
    pub level: i64,
    pub coord: Coord2,
    #[serde(rename = "zip_code")]
    pub zip_code: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coord2 {
    pub lon: String,
    pub lat: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopArea {
    pub id: String,
    pub coord: Coord3,
    pub label: String,
    pub name: String,
    #[serde(rename = "administrative_regions")]
    pub administrative_regions: Vec<AdministrativeRegion3>,
    pub timezone: String,
    #[serde(rename = "commercial_modes")]
    pub commercial_modes: Vec<CommercialMode>,
    #[serde(rename = "physical_modes")]
    pub physical_modes: Vec<PhysicalMode>,
    pub comment: Value,
    pub codes: Vec<Code>,
    pub lines: Vec<Line>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coord3 {
    pub lat: String,
    pub lon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdministrativeRegion3 {
    pub id: String,
    pub insee: String,
    pub name: String,
    pub label: String,
    pub level: i64,
    pub coord: Coord4,
    #[serde(rename = "zip_code")]
    pub zip_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coord4 {
    pub lon: String,
    pub lat: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommercialMode {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalMode {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Code {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    pub id: String,
    pub name: String,
    pub code: String,
    #[serde(rename = "commercial_mode")]
    pub commercial_mode: CommercialMode2,
    #[serde(rename = "physical_modes")]
    pub physical_modes: Vec<PhysicalMode2>,
    pub network: Network,
    pub color: String,
    #[serde(rename = "text_color")]
    pub text_color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommercialMode2 {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalMode2 {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Warning {
    pub id: String,
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedPublisher {
    pub id: String,
    pub license: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    #[serde(rename = "current_datetime")]
    pub current_datetime: String,
    pub timezone: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub href: String,
    pub templated: bool,
    pub rel: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
