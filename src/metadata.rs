use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Metadata {
    streams: Vec<Stream>,
    format: Format,
}

impl Metadata {
    pub fn new(path: String) -> Metadata {
        let s = Self::generate(path);
        serde_json::from_str(&s).expect("Unable to parse metadata.")
    }

    fn generate(path: String) -> String {
        let cmd = Command::new("ffprobe")
            .args(&[
                "-print_format",
                "json",
                "-show_format",
                "-show_streams",
                &path,
            ])
            .output()
            .expect(" Unable to generate metadata. ");

        let out = cmd.stdout;

        let s = String::from_utf8_lossy(&out);

        s.to_string()
    }
}

pub fn from_file() -> Metadata {
    // let s = std::fs::read_to_string("test.json").unwrap();
    // let m: Metadata = serde_json::from_str(&s).unwrap();
    // return m;
    todo!()
}

impl ToString for Metadata {
    fn to_string(&self) -> String {
        let mut top = format!(
            "= {} = \n\nFormat: {}\nDuration: {}s\nSize: {}mb\nBitrate: {}mbps\n#Streams: {}\n\n= Streams =\n",
            self.format.filename,
            self.format.format_long_name,
            parse_duration(self.format.duration.clone()),
            parse_size(self.format.size.clone()),
            parse_bitrate(self.format.bit_rate.clone()),
            self.format.nb_streams,
        );

        for s in self.streams.iter() {
            top += "\n";
            top += &s.to_string();
            top += "\n";
        }

        return top;
    }
}

#[derive(Deserialize, Default, Debug, Serialize, Clone)]
#[serde(default)]
pub struct Stream {
    index: usize,
    width: usize,
    height: usize,
    codec_type: Codec,
    codec_long_name: String,
    bit_rate: String,
    channels: usize,
    sample_rate: String,
    display_aspect_ratio: String,
    channel_layout: String,
}

impl ToString for Stream {
    fn to_string(&self) -> String {
        if self.codec_type == Codec::Video {
            format!(
                "{}. Stream - Video\nCodec: {}\nWidth: {}\nHeight: {}\nRatio:{}\nBitrate: {}",
                self.index,
                self.codec_long_name,
                self.width,
                self.height,
                self.display_aspect_ratio,
                parse_bitrate(self.bit_rate.clone())
            )
        } else {
            format!(
                "{}. Stream - Audio\nCodec: {}\nSample: {}\nLayout: {}\nBitrate: {}mbps",
                self.index,
                self.codec_long_name,
                self.sample_rate,
                self.channel_layout,
                parse_bitrate(self.bit_rate.clone())
            )
        }
    }
}

#[derive(Deserialize, Debug, Serialize, Eq, PartialEq, Clone)]
pub enum Codec {
    #[serde(rename(deserialize = "video"))]
    Video,
    #[serde(rename(deserialize = "audio"))]
    Audio,
}

impl Default for Codec {
    fn default() -> Self {
        Self::Video
    }
}

#[derive(Deserialize, Default, Debug, Serialize, Clone)]
#[serde(default)]
pub struct Format {
    filename: String,
    duration: String,
    size: String,
    format_long_name: String,
    bit_rate: String,
    nb_streams: usize,
}

fn parse_bitrate(bit: String) -> String {
    if let Ok(i) = bit.parse::<u64>() {
        format!("{:.2}", i as f64 / (1000.0 * 1000.0))
    } else {
        bit
    }
}

fn parse_duration(dur: String) -> String {
    let pts: Vec<&str> = dur.split(".").collect();
    if let Some(s) = pts.get(0) {
        if let Ok(i) = s.parse::<u64>() {
            return i.to_string();
        } else {
            dur
        }
    } else {
        dur
    }
}

fn parse_size(dur: String) -> String {
    if let Ok(i) = dur.parse::<u64>() {
        format!("{:.2}", (i as f64 / 1000000.0))
    } else {
        dur
    }
}
