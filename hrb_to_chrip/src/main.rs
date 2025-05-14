use camino::Utf8PathBuf as PathBuf;
use clap::Parser;
use clap_derive::Parser;

#[derive(Debug, Parser)]
struct Args {
    input_csv: PathBuf,
    output_csv: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let output_csv = match args.output_csv {
        Some(path) => path,
        None => {
            let base = args.input_csv.file_stem().expect("Failed to get file stem");
            let fp = format!("{}.chirp.csv", base);

            args.input_csv.with_file_name(fp)
        }
    };

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(&args.input_csv)
        .expect("Failed to open input CSV file");

    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(&output_csv)
        .expect("Failed to open output CSV file");

    // Location,Name,Frequency,Duplex,Offset,Tone,rToneFreq,cToneFreq,DtcsCode,DtcsPolarity,RxDtcsCode,CrossMode,Mode,TStep,Skip,Power,Comment,URCALL,RPT1CALL,RPT2CALL,DVCODE
    let mut headers = csv::StringRecord::new();
    headers.push_field("Location");
    headers.push_field("Name");
    headers.push_field("Frequency");
    headers.push_field("Duplex");
    headers.push_field("Offset");
    headers.push_field("Tone");
    headers.push_field("rToneFreq");
    headers.push_field("cToneFreq");
    headers.push_field("DtcsCode");
    headers.push_field("DtcsPolarity");
    headers.push_field("RxDtcsCode");
    headers.push_field("CrossMode");
    headers.push_field("Mode");
    headers.push_field("TStep");
    headers.push_field("Skip");
    headers.push_field("Power");
    headers.push_field("Comment");
    headers.push_field("URCALL");
    headers.push_field("RPT1CALL");
    headers.push_field("RPT2CALL");
    headers.push_field("DVCODE");

    writer
        .write_record(&headers)
        .expect("Failed to write headers");

    for result in reader.records() {
        let record = result.expect("Failed to read record");
        let mut new_record = csv::StringRecord::new();

        let num = record.get(0).unwrap_or("");
        if num == "" {
            // Skip empty lines
            continue;
        }

        // skip restricted
        let assignment = record.get(3).unwrap_or("");
        if assignment.starts_with("RESTRICTED") {
            continue;
        }

        //"Ch #",Function,"Channel Name/Trunked Radio System Talkgroup",Assignment,RX Freq      N or W,RX Tone/NAC,TX Freq      N or W,"TX Tone/NAC","Mode A or D",Remarks
        //1,Start Area Logistics,S1,Hopkinton Wide Area,447.775 W,88.5,442.775 W,88.5,A,Hopkinton (2025 Freq Change)
        new_record.push_field(num);
        new_record.push_field(record.get(2).unwrap_or(""));
        let rx_freq = record
            .get(4)
            .unwrap_or("")
            .replace(" W", "")
            .replace(" N", "");
        let rx_freq = rx_freq.parse::<f64>().unwrap_or(0.0);
        if !record.get(4).unwrap_or("").ends_with(" W") {
            panic!("Unknown RX width")
        };

        let tx_freq = record
            .get(6)
            .unwrap_or("")
            .replace(" W", "")
            .replace(" N", "");
        let tx_freq = tx_freq.parse::<f64>().unwrap_or(0.0);
        if !record.get(6).unwrap_or("").ends_with(" W") {
            panic!("Unknown TX width")
        };

        let offset = tx_freq - rx_freq;

        let rx_tone = record.get(5).unwrap_or("");
        let tx_tone = record.get(7).unwrap_or("");

        // fixme: consider using decimal math so we don't have inaccurate decimals
        let rx_freq_str = format!("{:.5}", rx_freq);
        //let tx_freq_str = format!("{:.5}", tx_freq);
        let offset_str = format!("{:.5}", offset);

        // We're assuming we have only CTCSS, no DCS handling.

        new_record.push_field(rx_freq_str.as_str());
        // Duplex
        new_record.push_field("");
        // Offset
        new_record.push_field(offset_str.as_str());
        // Tone (could be "Tone" or "TSQL")
        new_record.push_field("TSQL");
        // rToneFreq
        new_record.push_field(rx_tone);
        // cToneFreq
        new_record.push_field(tx_tone);
        // DtcsCode
        new_record.push_field("023");
        // DtcsPolarity
        new_record.push_field("NN");
        // RxDtcsCode
        new_record.push_field("023");
        // CrossMode
        new_record.push_field("Tone->Tone");
        // Mode
        new_record.push_field("FM"); // Assuming 25kHz for all channels
        // TStep (arbitrary)
        new_record.push_field("5.0");
        // Skip
        new_record.push_field("");
        // Power
        new_record.push_field("50W");
        // Comment
        // TODO: merge some other fields
        let function = record.get(1).unwrap_or("");
        let remarks = record.get(9).unwrap_or("");
        let comment = format!("{} | {} | {}", function, assignment, remarks);
        new_record.push_field(&comment);

        // URCALL
        new_record.push_field("");
        // RPT1CALL
        new_record.push_field("");
        // RPT2CALL
        new_record.push_field("");
        // DVCODE
        new_record.push_field("");

        writer
            .write_record(&new_record)
            .expect("Failed to write record");
    }

    writer.flush().expect("Failed to flush writer");
}
