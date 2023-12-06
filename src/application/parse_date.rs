//!  # Parse the filename title to get the date

use chrono::NaiveDate;

pub fn parse(title: &str) -> String {
    let first = title.chars().nth(0).unwrap_or('*');
    if first.is_ascii_digit() {
        parse_manual(title)
    } else {
        parse_shadowplay(title)
    }
}

fn parse_manual(title: &str) -> String {
    let mut res = String::from("");
    for (i, part) in title.split("-").enumerate() {
        if i == 0 {
            // Part 0: "10_11_23"
            let mut year = 0;
            let mut month = 0;
            let mut day = 0;
            for (j, comp) in part.split("_").enumerate() {
                match j {
                    0 => day = comp.parse().unwrap_or(0),
                    1 => month = comp.parse().unwrap_or(0),
                    2 => year = comp.parse().unwrap_or(0),
                    _ => {}
                }
            }
            if year != 0 && month != 0 && day != 0 {
                // TODO: Error handling?
                let ts = NaiveDate::from_ymd_opt(year + 2000, month, day).unwrap();
                res = ts.format("%d-%m-%Y").to_string();
            }
        } else {
            break;
        }
    }

    return res;
}

fn parse_shadowplay(title: &str) -> String {
    let mut res = String::from("");
    for (i, part) in title.split(" ").enumerate() {
        if i == 2 {
            // 2023.12.05
            let mut year = 0;
            let mut month = 0;
            let mut day = 0;
            for (j, comp) in part.split(".").enumerate() {
                match j {
                    0 => year = comp.parse().unwrap_or(0),
                    1 => month = comp.parse().unwrap_or(0),
                    2 => day = comp.parse().unwrap_or(0),
                    _ => {}
                }
            }
            if year != 0 && month != 0 && day != 0 {
                let ts = NaiveDate::from_ymd_opt(year, month, day).unwrap();
                res = ts.format("%d-%m-%Y").to_string();
            }
        }
    }
    return res;
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_shadowplay() {
        let title = "Counter-strike 2 2023.12.05 - 22.14.07.02.DVR.mp4";
        assert_eq!(parse(title), "05-12-2023");
    }

    #[test]
    fn test_parse_manual() {
        let title = "10_11_23-Inferno-Retake_Clutch_AWP.mp4";
        assert_eq!(parse(title), "10-11-2023");
    }
}
