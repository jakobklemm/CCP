//! Processor

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use uuid::Uuid;

use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Job {
    key: Uuid,
    start: Timestamp,
    end: Timestamp,
    input: String,
    output: String,
    language: Language,
}

#[derive(Clone, Debug)]
pub enum Language {
    EN,
    DE,
}

#[derive(Clone, Debug)]
pub struct Report {
    transcript: String,
    size: f64,
}

impl Report {
    fn new(path: String, transcript: String) -> Result<Self> {
        // TODO: get size
        Ok(Self {
            transcript,
            size: 0.0,
        })
    }
}

pub enum Status {
    /// Has to be between 0 and 100
    Progress(i32),
    Completed(Box<Report>),
}

impl Job {
    pub fn new(
        start: impl ToString,
        end: impl ToString,
        input: impl ToString,
        output: impl ToString,
    ) -> Option<Self> {
        Some(Self {
            key: Uuid::new_v4(),
            start: Timestamp::from_str(start)?,
            end: Timestamp::from_str(end)?,
            input: input.to_string(),
            output: output.to_string(),
            language: Language::default(),
        })
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }

    pub fn execute(self) -> Receiver<Status> {
        let (snd, recv) = channel();
        let _ = thread::spawn(move || {
            let _ = snd.send(Status::Progress(0)).unwrap();

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
                .arg(self.input)
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
                .arg(self.output)
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
                    let s = Status::Progress(parse_percentage(perc));
                    let _ = snd.send(s).unwrap();
                }
            }

            cmd.wait().unwrap();
            let t = String::from("-");
            let e = String::from("_");
            let s = Status::Completed(Box::new(Report::new(e, t).unwrap()));
            let _ = snd.send(s).unwrap();
        });
        recv
    }
}

fn parse_percentage(perc: f32) -> i32 {
    (perc * 100.0) as i32
}

#[derive(Debug, Default, Clone, Copy)]
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
