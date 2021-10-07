#![warn(rust_2018_idioms, missing_debug_implementations)]
use snafu::Snafu;

pub mod sparse_mem;
pub mod anytone_ht;
pub mod icom_id51a;
pub mod ne_repeater;


// FIXME: this shouldn't have public context constructors
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum FreqmError {
     #[snafu(display("CSV missing field(s): have {}", field_num))]
    FieldMissing {
        field_num: usize,
    },

    #[snafu(display("output frequency {:?} is not a decimal", output_freq))]
    OutputFreqParseFailure {
        output_freq: String,
    },

    #[snafu(display("offset kind {:?} unrecognized", offset_kind))]
    InvalidOffsetKind {
        offset_kind: String,
    },

    #[snafu(display("frequency {} is not in a known band", freq))]
    FreqNotInAnyBand {
        freq: decimal::d128
    },

    #[snafu(display("comment parse failed: {:?}", comment))]
    CommentParse {
        comment: String,
    }
}

#[derive(Debug)]
pub struct Repeater {
    /// Frequency on which the repeater transmits
    output_freq: decimal::d128,

    /// Frequency on which the repeater receives input
    ///
    /// optional because some datasets don't include this
    input_freq: Option<decimal::d128>,
}

impl Repeater {
    pub fn freq_as_offset(&self) -> (f64, f64) {
        todo!()
    }

    pub fn bandwidth(&self) -> f64 {
        todo!()
    }

    pub fn mode(&self) -> usize {
        todo!()
    }

    /// Note: codes are limited by `mode`, consider if we should have a `mode` which contains the
    /// code info
    pub fn code_in(&self) -> Option<usize> {
        todo!()
    }

    pub fn code_out(&self) -> Option<usize> {
        todo!()
    }

}

