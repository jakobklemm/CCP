//! Processor

use polodb_core::bson::doc;

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use std::{fs, thread};

use super::id::Id;
use super::job::Job;
use super::status::Status;
use super::timestamp::Timestamp;

use anyhow::{anyhow, Result};

use crate::DATABASE;

pub fn execute(job: Job) -> Receiver<Status> {
    let (sender, receiver) = channel();

    let id = Id::default();

    let _ = thread::spawn(move || {
        if let Ok(_) = first_pass(id.clone(), job.clone(), sender.clone()) {
            if let Ok(t) = second_pass(id.clone(), job.clone(), sender.clone()) {
                let _ = third_pass(id.clone(), job, sender.clone(), t);
                let _ = sender.send(Status::Complete(id));
            }
        }
    });

    receiver
}

fn first_pass(id: Id, job: Job, snd: Sender<Status>) -> Result<()> {
    let _ = snd.send(Status::First(0)).unwrap();

    let duration = job.end() - job.start();

    let pipe = Stdio::piped();
    let err = Stdio::piped();

    let mut cmd = Command::new("ffmpeg")
        .arg("-hide_banner")
        .arg("-v")
        .arg("quiet")
        .arg("-stats")
        .arg("-ss")
        .arg(job.start().to_string())
        .arg("-i")
        .arg(job.get_file())
        .arg("-c:v")
        .arg("copy")
        .arg("-c:a")
        .arg("aac")
        .arg("-filter_complex")
        .arg("amerge=inputs=2")
        .arg("-crf")
        .arg("10")
        .arg("-t")
        .arg(duration.to_string())
        .arg("-progress")
        .arg("/dev/stdout")
        .arg(id.temp_path().unwrap())
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
        if let Ok(ts) = Timestamp::from_str(times) {
            let perc = ts / duration;
            let parsed = parse_percentage(perc);
            let s = Status::First(parsed);
            let _ = snd.send(s).unwrap();
        }
    }

    let code = cmd.wait().unwrap();
    if code.success() {
        let _ = snd.send(Status::First(100));
        Ok(())
    } else {
        Err(anyhow!("pass 1 failed"))
    }
}

// Returns transcript string
fn second_pass(id: Id, job: Job, snd: Sender<Status>) -> Result<String> {
    let _ = snd.send(Status::Second(0)).unwrap();

    let duration = job.end() - job.start();

    let mut cmd = Command::new("whisper")
        .arg(id.temp_path().unwrap())
        .arg("--language")
        .arg("German")
        .arg("--model")
        .arg("medium")
        .arg("-o")
        .arg(id.temp_dir()?)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = cmd.stdout.as_mut().unwrap();
    let reader = BufReader::new(stdout);
    let lines = reader.lines();

    for i in 1..10 {
        let _ = snd.send(Status::Second(i as u16));
        thread::sleep(Duration::from_millis(125));
    }

    for (_i, line) in lines.enumerate() {
        // println!("LINE: {:?}", line);
        if line.is_err() {
            continue;
        }
        let line = line.unwrap();
        if let Some(ts) = get_segment(&line) {
            let perc = ts / duration;
            let parsed = parse_percentage(perc);
            // stupid UI enhancement
            if parsed <= 12 {
                continue;
            }
            let _ = snd.send(Status::Second(parsed));
        }
    }

    let f = std::fs::read_to_string(id.text_path()?)?;
    let _ = snd.send(Status::Second(100));
    // println!("{:?}", f);

    Ok(f)
}

fn third_pass(id: Id, job: Job, snd: Sender<Status>, text: String) -> Result<()> {
    // Tasks:
    // 1. Create Entry object
    // 2. Move media file
    // 3. Copy SRT file
    // 4. Write Entry to all three DBs
    // 5. Copy source file to source folder

    let _ = snd.send(Status::Third(0))?;

    let entry = job.to_entry(id.clone(), text)?;
    let from = id.temp_path()?;
    let to = id.data_path()?;

    // move
    let _ = fs::rename(from, to)?;

    // println!("FIRST MOVE: {:?}", r);

    let _ = snd.send(Status::Third(25))?;

    let from = id.srt_path()?;
    let to = id.srt_out()?;
    let _ = fs::rename(from, to)?;

    let _ = snd.send(Status::Third(50))?;

    let json = serde_json::to_string_pretty(&entry)?;
    let jpath = id.meta_path()?;
    let _ = fs::write(jpath, json)?;

    let _ = snd.send(Status::Third(75))?;

    let _ = DATABASE.insert_indexed(entry);

    let _ = snd.send(Status::Third(80))?;

    // check if source exists
    // let source = format!("{}/source/{}", ROOT.as_str(), job.get_file());
    // if let Ok(_f) = File::open(&source) {
    //     // file already exists
    // } else {
    //     let from = format!("{}/ingest/{}", ROOT.as_str(), job.get_file());
    //     let _ = fs::write(from, source)?;
    // }

    // mark job as done
    let _ = DATABASE.update_many::<Job>(
        doc! {"uid": job.uid},
        doc! {
            "$set": doc! {"done": true}
        },
    )?;

    Ok(())
}

fn parse_percentage(perc: f32) -> u16 {
    (perc * 100.0) as u16
}

fn get_timestamp(line: String) -> Option<String> {
    let parts: Vec<&str> = line.split("=").collect();
    if parts.len() != 2 {
        return None;
    }
    if parts.get(0).unwrap().to_string() != "out_time" {
        return None;
    }
    // let p = Timestamp::from_str(parts.get(1).unwrap());
    return Some(parts.get(1).unwrap().to_string());
}

fn get_segment(line: &str) -> Option<Timestamp> {
    let first = line.split(" ").nth(0)?;
    let mut found = String::from("00:");
    for (i, part) in first.split(":").enumerate() {
        match i {
            0 => {
                // minutes
                if let Some(c) = part.chars().nth(1) {
                    if c.is_ascii_digit() {
                        // pure perfection
                        found += part.split("[").nth(1).unwrap_or("");
                    }
                }
            }
            1 => {
                if let Some(scd) = part.split(".").nth(0) {
                    found += &format!(":{}", scd);
                }
                if let Some(milis) = part.split(".").nth(1) {
                    found += &format!(".{}", milis);
                } else {
                    found += ".00";
                }
            }
            _ => {}
        }
    }

    if let Ok(t) = Timestamp::from_str(found) {
        Some(t)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whisper_timestamp() {
        let line = "[00:37.000 --> 00:38.000]  Ich komme bei.";
        assert_eq!(
            get_segment(line).unwrap(),
            Timestamp::from_str("00:00:37.000").unwrap()
        );
    }
}
