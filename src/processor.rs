//! Processor

use crate::entry::Id;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use uuid::Uuid;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    key: Uuid,
    start: Timestamp,
    end: Timestamp,
    input: String,
    output: Id,
    language: Language,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Language {
    EN,
    DE,
}

impl Language {
    fn from_str(s: &str) -> Language {
        match s {
            "en" => Self::EN,
            _ => Self::DE,
        }
    }
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::EN => String::from("German"),
            Language::DE => String::from("Language"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    transcript: Vec<String>,
    size: f64,
}

impl Report {
    fn new(path: String, transcript: Vec<String>) -> Result<Self> {
        // TODO: get size
        Ok(Self {
            transcript,
            size: 0.0,
        })
    }
}

pub enum Status {
    /// Has to be between 0 and 100
    Media(i32),
    Text(i32),
    Completed(Box<Report>),
}

impl Job {
    pub fn new(
        start: impl ToString,
        end: impl ToString,
        input: impl ToString,
        output: Id,
    ) -> Option<Self> {
        Some(Self {
            key: Uuid::new_v4(),
            start: Timestamp::from_str(start)?,
            end: Timestamp::from_str(end)?,
            input: input.to_string(),
            output,
            language: Language::default(),
        })
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }

    pub fn execute(self) -> Receiver<Status> {
        let (snd, recv) = channel();
        let _ = thread::spawn(move || {
            if self.first_pass(snd.clone()) {
                if let Ok(rep) = self.second_pass(snd.clone()) {
                    let _ = snd.send(Status::Completed(Box::new(rep))).unwrap();
                }
            }
        });
        recv
    }

    fn second_pass(&self, snd: Sender<Status>) -> Result<Report> {
        let _ = snd.send(Status::Text(0)).unwrap();

        let mut cmd = Command::new("whisper")
            .arg(self.output.temp_path().unwrap())
            .arg("--language")
            .arg("German")
            .arg("--model")
            .arg("medium")
            .arg("-o")
            .arg(self.output.temp_dir())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        let stdout = cmd.stdout.as_mut().unwrap();
        let reader = BufReader::new(stdout);
        let lines = reader.lines();

        for (i, line) in lines.enumerate() {
            // println!("LINE: {:?}", line);
            let _ = snd.send(Status::Text(i as i32)).unwrap();
        }

        let f: Vec<String> = std::fs::read_to_string(self.output.transcript())
            .unwrap()
            .split("\n")
            .map(String::from)
            .collect();

        Ok(Report::new(String::new(), f).unwrap())
    }

    fn first_pass(&self, snd: Sender<Status>) -> bool {
        let _ = snd.send(Status::Media(0)).unwrap();

        let duration = self.end - self.start;

        let pipe = Stdio::piped();
        let err = Stdio::piped();

        let mut cmd = Command::new("ffmpeg")
            .arg("-hide_banner")
            .arg("-v")
            .arg("quiet")
            .arg("-stats")
            .arg("-ss")
            .arg(self.start.clone().to_string())
            .arg("-i")
            .arg(self.input.clone())
            .arg("-c:v")
            .arg("copy")
            .arg("-c:a")
            .arg("aac")
            .arg("-filter_complex")
            .arg("amerge=inputs=2")
            .arg("-crf")
            .arg("10")
            .arg("-to")
            .arg(self.end.clone().to_string())
            .arg("-progress")
            .arg("/dev/stdout")
            .arg(self.output.temp_path().unwrap())
            .stdout(pipe)
            .stderr(err)
            .spawn()
            .unwrap();

        let stdout = cmd.stdout.as_mut().unwrap();
        let reader = BufReader::new(stdout);
        let lines = reader.lines();

        for line in lines {
            let times = match get_timestamp(line.unwrap()) {
                Some(t) => t,
                None => String::new(),
            };
            if let Some(ts) = Timestamp::from_str(times) {
                let curr = ts - self.start;
                let perc = curr / duration;
                // println!("PROGRESS: {:?}", perc);
                let s = Status::Media(parse_percentage(perc));
                let _ = snd.send(s).unwrap();
            }
        }

        let code = cmd.wait().unwrap();
        code.success()
    }
}

fn parse_percentage(perc: f32) -> i32 {
    (perc * 100.0) as i32
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Timestamp {
    hours: i32,
    minutes: i32,
    seconds: i32,
    millis: i32,
}

use std::ops::{Div, Sub};

impl Sub for Timestamp {
    type Output = Timestamp;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            hours: self.hours - other.hours,
            minutes: self.minutes - other.minutes,
            seconds: self.seconds - other.seconds,
            millis: self.millis - other.millis,
        }
    }
}

impl Div for Timestamp {
    type Output = f32;

    fn div(self, rhs: Self) -> Self::Output {
        self.to_seconds() as f32 / rhs.to_seconds() as f32
    }
}

impl Timestamp {
    fn to_seconds(&self) -> i32 {
        3600 * self.hours + 60 * self.minutes + self.seconds
    }

    pub fn from_str(line: impl ToString) -> Option<Self> {
        let binding = line.to_string();
        let parts = binding.split(":");
        let mut ts = Timestamp::default();
        for (i, part) in parts.into_iter().enumerate() {
            match i {
                0 => {
                    // println!("{:?}", part);
                    // ts.hours = part.parse().unwrap();
                    if let Ok(h) = part.parse() {
                        ts.hours = h;
                    } else {
                        return None;
                    }
                }
                1 => {
                    // ts.minutes = part.parse().unwrap();
                    if let Ok(m) = part.parse() {
                        ts.minutes = m;
                    } else {
                        return None;
                    }
                }
                2 => {
                    let inner_parts = part.split(".");
                    for (j, p) in inner_parts.into_iter().enumerate() {
                        match j {
                            0 => {
                                // ts.seconds = p.parse().unwrap();
                                if let Ok(s) = p.parse() {
                                    ts.seconds = s;
                                } else {
                                    return None;
                                }
                            }
                            1 => {
                                // ts.millis = p.parse().unwrap();
                                if let Ok(m) = p.parse() {
                                    ts.millis = m;
                                } else {
                                    return None;
                                }
                            }
                            _ => {
                                return None;
                            }
                        }
                    }
                }
                _ => {
                    return None;
                }
            }
        }
        // println!("{:?}", ts);
        Some(ts)
    }
}

impl ToString for Timestamp {
    fn to_string(&self) -> String {
        format!(
            "{}:{}:{}.{}",
            self.hours, self.minutes, self.seconds, self.millis
        )
    }
}

fn get_timestamp(line: String) -> Option<String> {
    let parts: Vec<&str> = line.split("=").collect();
    if parts.len() != 2 {
        return None;
    }
    if parts.get(0).unwrap().to_string() != "out_time" {
        return None;
    }
    let p = Timestamp::from_str(parts.get(1).unwrap());
    return Some(parts.get(1).unwrap().to_string());
}

impl Default for Language {
    fn default() -> Self {
        Self::DE
    }
}
