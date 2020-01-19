use libc::{c_char, c_int, size_t};
use std::ptr;

#[derive(Debug)]
pub enum EarwaxErrorCode {
    Io = 100,
    AudioStreamNotFound,
    DecoderNotFound,
    UnableToOpenDecoder,
    Unknown,
}

impl EarwaxErrorCode {
    pub fn from_c_int(code: c_int) -> Self {
        match code {
            100 => EarwaxErrorCode::Io,
            101 => EarwaxErrorCode::AudioStreamNotFound,
            102 => EarwaxErrorCode::DecoderNotFound,
            103 => EarwaxErrorCode::UnableToOpenDecoder,
            _ => EarwaxErrorCode::Unknown,
        }
    }
}

impl From<c_int> for EarwaxErrorCode {
    fn from(code: c_int) -> EarwaxErrorCode {
        EarwaxErrorCode::from_c_int(code)
    }
}

pub enum EarwaxContext {}

#[repr(C)]
pub struct EarwaxRational {
    pub num: i64,
    pub den: i64,
}

impl EarwaxRational {
    pub fn new() -> Self {
        EarwaxRational { num: 1, den: 1 }
    }
}

#[repr(C)]
pub struct EarwaxInfo {
    pub bitrate: c_int,
    pub sample_rate: c_int,
    pub start_time: i64,
    pub duration: i64,
    pub time_base: EarwaxRational,
}

impl EarwaxInfo {
    pub fn new() -> Self {
        EarwaxInfo {
            bitrate: 0,
            sample_rate: 0,
            start_time: 0,
            duration: 0,
            time_base: EarwaxRational::new(),
        }
    }
}

#[repr(C)]
pub struct EarwaxChunk {
    pub data: *mut c_char,
    pub size: size_t,
    pub time: i64,
}

impl EarwaxChunk {
    pub fn new() -> Self {
        EarwaxChunk {
            data: ptr::null_mut(),
            size: 0,
            time: 0,
        }
    }
}

#[link(name="earwax")]
#[link(name="avcodec")]
#[link(name="avformat")]
#[link(name="swresample")]
#[link(name="avutil")]
#[link(name="pthread")]
extern "C" {
    pub fn earwax_init();
    pub fn earwax_shutdown();
    pub fn earwax_new(ctx: *mut *mut EarwaxContext, url: *const c_char) -> c_int;
    pub fn earwax_drop(ctx: *mut *mut EarwaxContext);
    pub fn earwax_get_info(ctx: *const EarwaxContext, info: *mut EarwaxInfo);
    pub fn earwax_spit(ctx: *mut EarwaxContext, chunk: *mut EarwaxChunk) -> c_int;
    pub fn earwax_seek(ctx: *mut EarwaxContext, pts: i64) -> c_int;

    // Other ffmpeg functions.
    pub fn av_log_get_level() -> c_int;
    pub fn av_log_set_level(level: c_int);
}
