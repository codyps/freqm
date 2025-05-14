//! Interfaces to the radioid.net web service.

use serde::{Deserialize, Serialize};

/// https://radioid.net/static/rptrs.json
#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RptrsResponse {
    pub rptrs: Vec<Rptr>,
}

//
#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Rptr {
    pub locator: u64,
    // typical locator & id are the same.
    pub id: u64,

    // "ACTIVE"
    pub status: String,

    pub callsign: String,

    // "SAN JOSE"
    pub city: String,
    // "California"
    pub state: String,
    // "United States"
    pub country: String,

    // "441.82500"
    pub frequency: String,
    // 1
    pub color_code: u32,

    // "+5.000"
    pub offset: String,
    // "Peer"
    pub assigned: String,

    // "TS1 TS2"
    pub ts_linked: String,

    pub trustee: String,

    pub map_info: String,
    pub map: u32,

    // NOTE: needs normalization. "BM" vs "Brandmeister" vs "BrandMeister"
    pub ipsc_network: String,
}
