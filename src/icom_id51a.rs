//! Icom ID-51a (and plus and plus2)
//!
//! - `icf` (icom configuration file) file stores settings
//! - various csv files 
//!
//! Icom provides downlaods for some configuration files definiting repeaters: https://www.icomjapan.com/support/firmware_driver/2444/


/// `IRNAID51.csv`
///
/// Header:
/// ```norust
/// Group No,Group Name,Name,Sub Name,Repeater Call Sign,Gateway Call Sign,Frequency,Dup,Offset,Mode,TONE,Repeater Tone,RPT1USE,Position,Latitude,Longitude,UTC Offset
/// 4,Canada,Saint John,New Brunswick,VE9SJN C,VE9SJN G,145.2900,DUP-,0.6,DV,OFF,82.5Hz,Yes,Approximate,45.32,-66.06,-04:00
/// ```
///
#[derive(Debug, Clone)]
pub struct ChannelLine {
    pub group_number: u64,
    pub group_name: String,
    pub name: String,
    pub sub_name: String,
    pub repeated_call_sign: String,
    pub gateway_call_sign: String,
    pub frequency: f64,
    pub dup: String,
    pub offset: f64,
    pub mode: String,
    pub tone: String,
    pub repeater_tone: String,
    pub rpt1use: String,
    pub position: String,
    pub latitude: f64,
    pub longitude: f64,
    pub utc_offset: String,
}


