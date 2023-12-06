//! Processor

use chrono::{Local, NaiveDate};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use super::timestamp::Timestamp;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    start: Timestamp,
    end: Timestamp,
    pub title: String,
    pub desc: String,
    timestamp: NaiveDate,
    tags: Vec<String>,
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
    pub fn from_str(s: &str) -> Language {
        let s = s.trim();
        match s {
            "en" => Self::EN,
            _ => Self::DE,
        }
    }

    pub fn from_input(lines: &[String]) -> Language {
        for l in lines {
            return Self::from_str(l);
        }
        Self::default()
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
        title: impl ToString,
        desc: impl ToString,
        timestamp: impl ToString,
        tags: Vec<String>,
    ) -> Option<Self> {
        Some(Self {
            start: Timestamp::from_str(start)?,
            end: Timestamp::from_str(end)?,
            input: input.to_string(),
            title: title.to_string(),
            desc: desc.to_string(),
            timestamp: NaiveDate::parse_from_str(&timestamp.to_string(), "%d-%m-%Y")
                .unwrap_or(Local::now().date_naive()),
            tags,
            output: Id::default(),
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
            .arg(self.output.temp_dir()?)
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

        let f: Vec<String> = std::fs::read_to_string("")
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

use super::id::Id;

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
