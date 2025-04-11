// Supported by Kenwood supplied MCP (memory control program) software as an
// import format
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ArrlTravelPlusRow {
    sequence_number: String,
    country: String,
    region: String,
    state: String,
    location: String,
    output_frequency: String,
    input_frequency: String,
    call_sign: String,
    repeater_notes: String,
    ctcss_tones: String,
    sponsor: String,
}

// Chrip format, offset based.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ChirpRow {
    location: String,
    name: String,
    frequency: String,
    duplex: String,
    offset: String,
    tone: String,
    r_tone_freq: String,
    c_tone_freq: String,
    dtcs_code: String,
    dtcs_polarity: String,
    rx_dtcs_code: String,
    cross_mode: String,
    mode: String,
    t_step: String,
    skip: String,
    power: String,
    comment: String,
    ur_call: String,
    rpt1_call: String,
    rpt2_call: String,
    dv_code: String,
}

// tsv (tab seperated)
// Wn	World Region	Cn	Country	Gn	Group	Callsign	Gateway	Lockout	Name	Sub Name	Frequency	Shift	Offset	Mode	Uplink Tone	Downlink Tone	Position	Lat DD	Lat MM.mm	N/S	Lon DDD	Lon MM.mm	E/W	Time Zone	TH-D74A	TH-D74E	TH-D74	Aux 1	Aux 2	Aux 3
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct KenwoodTh74aRow {
    wn: String,
    world_region: String,
    cn: String,
    country: String,
    gn: String,
    group: String,
    callsign: String,
    gateway: String,
    lockout: String,
    name: String,
    sub_name: String,
    frequency: String,
    shift: String,
    offset: String,
    mode: String,
    uplink_tone: String,
    downlink_tone: String,
    position: String,
    lat_dd: String,
    lat_mm_mm: String,
    n_s: String,
    lon_ddd: String,
    lon_mm_mm: String,
    e_w: String,
    time_zone: String,
}

/// Boston Marathon ICS (Incident Command System) format, exported from the PDF using Tabula
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct BostonMarathonIcsRow {
    location: String,
    name: String,
    frequency: String,
    duplex: String,
    offset: String,
    tone: String,
    r_tone_freq: String,
    c_tone_freq: String,
    dtcs_code: String,
    dtcs_polarity: String,
    rx_dtcs_code: String,
    cross_mode: String,
    mode: String,
    t_step: String,
    skip: String,
    power: String,
    comment: String,
    ur_call: String,
    rpt1_call: String,
    rpt2_call: String,
    dv_code: String,
}
