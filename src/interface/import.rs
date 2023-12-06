//! # Import

use std::{
    fs::File,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use crate::application::{tag::Tag, Metadata};
use crate::{
    interface::{Render, TextArea},
    update::control,
    util, DATABASE, ROOT,
};
use chrono::{DateTime, Local, SecondsFormat};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::Rect,
    style::{Color, Modifier, Style},
    widgets::{block::Title, Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

const INPUTS: usize = 7;

/// TODO: Add Tags list and file tree.
#[derive(Debug, Clone)]
pub struct Import {
    // The currently selected input field
    selected: usize,
    // Start timestamp field
    start: TextArea,
    // End timestamp field
    end: TextArea,
    // Title input field
    title: TextArea,
    // Description input field
    description: TextArea,
    // Tags input field
    tags: TextArea,
    //  Langugae input field
    language: TextArea,
    // File browser on left, (Filename, File metadata)
    files: ItemList<(String, Metadata)>,
    // The current timestamp (real world), maybe parsed by file name
    timestamp: TextArea,
    // Selector list on bottom
    tagslist: ItemList<Tag>,
    // System state
    meta: Meta,
}

#[derive(Debug, Clone)]
struct Meta {
    // Is the input valid?
    valid: bool,
    // Should a popup be drawn
    popped: bool,
    // When was submitted
    moment: Instant,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            valid: false,
            popped: false,
            moment: Instant::now(),
        }
    }
}

use std::fs;

use super::list::ItemList;

impl Render for Import {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        if self.meta.popped {
            if self.meta.moment.elapsed().as_secs_f64() > 1.25 {
                self.meta.popped = false;
            } else {
                let (text, style) = if self.meta.valid {
                    (format!("Job created!"), Style::default().fg(Color::Green))
                } else {
                    (format!("Invalid inputs!"), Style::default().fg(Color::Red))
                };
                let block = Block::default()
                    .title(" Action ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .style(style.clone());
                let center = util::center(10, 15, area);
                f.render_widget(Paragraph::new(text).block(block).style(style), center);
                return;
            }
        }

        let accept = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Green));

        match self.selected {
            0 => {
                self.reset_all();
                self.timestamp.set_block(accept.title(" Timestamp "));
            }
            1 => {
                self.reset_all();
                self.start.set_block(accept.title(" Start Time "));
            }
            2 => {
                self.reset_all();
                self.end.set_block(accept.title(" End Time "));
            }
            3 => {
                self.reset_all();
                self.title.set_block(accept.title(" Enter Title "));
            }
            4 => {
                self.reset_all();
                self.language.set_block(accept.title(" Language "));
            }
            5 => {
                self.reset_all();
                self.description.set_block(accept.title(" Description "));
            }
            6 => {
                self.reset_all();
                self.tags.set_block(accept.title(" Enter Tags "));
            }
            _ => {}
        }

        let layout = Layout::default()
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .direction(Direction::Horizontal)
            .split(area);

        self.render_tree(f, layout[0]);
        self.render_input(f, layout[1]);
        self.render_meta(f, layout[2]);
    }

    /// TODO: Handle inner swap to lower block
    fn input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab => {
                self.focus_next();
            }
            KeyCode::BackTab => {
                self.focus_prev();
            }
            KeyCode::Down if control(&key) => {
                self.focus_next();
            }
            KeyCode::Up if control(&key) => {
                self.focus_prev();
            }
            KeyCode::Char('n') if key.modifiers == KeyModifiers::CONTROL => {
                self.files.next();
                self.reset_timestamp();
                // TODO: Update timestamp (maybe?)
            }
            KeyCode::Char('p') if key.modifiers == KeyModifiers::CONTROL => {
                self.files.previous();
                self.reset_timestamp();
                // TODO: Update timestamp (maybe?)
            }
            KeyCode::Char('s') if key.modifiers == KeyModifiers::CONTROL => {
                // self.save_job();
            }
            KeyCode::Char('j') if key.modifiers == KeyModifiers::CONTROL => {
                self.tagslist.next();
            }
            KeyCode::Char('k') if key.modifiers == KeyModifiers::CONTROL => {
                self.tagslist.previous();
            }
            KeyCode::Enter => {
                // Submit selected tag
                if let Some(tag) = self.tagslist.get() {
                    let _ = self.tags.insert_str(tag.to_string());
                    self.selected = 6;
                }
            }
            _ => {
                match self.selected {
                    0 => self.timestamp.input(key),
                    1 => self.start.input(key),
                    2 => self.end.input(key),
                    3 => self.title.input(key),
                    4 => self.language.input(key),
                    5 => self.description.input(key),
                    6 => self.tags.input(key),
                    _ => false,
                };
            }
        }
    }
}

impl Import {
    fn reset_timestamp(&mut self) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let style = Style::default();

        let mut timestamp = TextArea::default();
        timestamp.set_placeholder_text("%d-%m-%Y");
        timestamp.set_block(block);
        timestamp.set_cursor_line_style(style);

        if let Some((file, _meta)) = self.files.get() {
            timestamp.insert_str(parse(&file));
        }
        self.timestamp = timestamp;
    }

    //     fn save_job(&mut self) {
    //         self.id = String::new();
    //         self.popped = true;
    //         self.moment = Instant::now();
    //         let start = Timestamp::from_input(self.start.lines());
    //         let end = Timestamp::from_input(self.end.lines());
    //         let timestamp = {
    //             let first = self.timestamp.lines().get(0);
    //             if first.is_none() {
    //                 self.valid = false;
    //                 return;
    //             }
    //             first.unwrap()
    //         };
    //         // TODO: Parse Title properly
    //         let title = {
    //             let first = self.title.lines().get(0);
    //             if first.is_none() {
    //                 self.valid = false;
    //                 return;
    //             }
    //             first.unwrap()
    //         };
    //         let desc = {
    //             let first = self.desc.lines().get(0);
    //             if first.is_none() {
    //                 self.valid = false;
    //                 return;
    //             }
    //             first.unwrap()
    //         };
    //         let lang = Language::from_input(self.language.lines());
    //
    //         // 0. Check if file selected
    //         let file = {
    //             if let Some(ind) = self.get_path() {
    //                 ind
    //             } else {
    //                 self.valid = false;
    //                 return;
    //             }
    //         };
    //         // 1. Create job
    //         let mut job = {
    //             // TODO: Add tags
    //             if let Some(j) = Job::new(start, end, file, title, desc, timestamp, Vec::new()) {
    //                 self.valid = true;
    //                 j
    //             } else {
    //                 self.valid = false;
    //                 return;
    //             }
    //         };
    //         job.set_language(lang);
    //         // 2. Check validity
    //         if !self.valid {
    //             return;
    //         }
    //         // 3. Submit to DB
    //         let col = DATABASE.collection::<Job>("jobs");
    //         if let Err(_res) = col.insert_one(job) {
    //             self.valid = false;
    //             return;
    //         }
    //         // 4. Reset inputs
    //         let def = Self::default();
    //         self.language = def.language;
    //         self.start = def.start;
    //         self.end = def.end;
    //         self.title = def.title;
    //         self.selector = def.selector;
    //         // 5. Popup
    //         self.popped = true;
    //         self.moment = Instant::now();
    //     }

    fn reset_all(&mut self) {
        let block = Block::default()
            .style(Style::default().fg(Color::Gray))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        self.timestamp.set_block(block.clone().title(" Timestamp "));
        self.start.set_block(block.clone().title(" Start Time "));
        self.end.set_block(block.clone().title(" End Time "));
        self.title.set_block(block.clone().title(" Enter Title "));
        self.description
            .set_block(block.clone().title(" Description "));
        self.language.set_block(block.clone().title(" Language "));
        self.tags.set_block(block.clone().title(" Enter Tags "));
    }

    fn focus_next(&mut self) {
        self.selected = (self.selected + 1) % INPUTS;
    }

    fn focus_prev(&mut self) {
        self.selected = (INPUTS + self.selected - 1) % INPUTS;
    }

    fn render_tree(&mut self, f: &mut Frame, area: Rect) {
        let itms: Vec<ListItem> = self
            .files
            .items()
            .iter()
            .map(|x| {
                let t = format!("{}", x.0);
                ListItem::new(t).style(Style::default().fg(Color::Gray))
            })
            .collect();

        let list = List::new(itms)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .highlight_style(Style::default().fg(Color::Cyan));

        f.render_stateful_widget(list, area, &mut self.files.state);
    }

    fn render_input(&mut self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        self.render_upper(f, layout[0]);
        self.render_lower(f, layout[1]);
    }

    fn render_upper(&mut self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .direction(Direction::Horizontal)
            .split(area);

        self.render_simples(f, layout[0]);
        self.render_text(f, layout[1]);
    }

    fn get_title(&self) -> String {
        if let Some(s) = self.files.get() {
            s.0.to_string()
        } else {
            String::from("NO FILE SELECTED")
        }
    }

    fn get_meta(&self) -> Option<Metadata> {
        if let Some(i) = self.files.get() {
            Some(i.1.clone())
        } else {
            None
        }
    }

    fn get_path(&self) -> Option<String> {
        let path = format!("{}/ingest/{}", ROOT.as_str(), self.get_title());
        if let Ok(_f) = File::open(path.clone()) {
            Some(path)
        } else {
            None
        }
    }

    fn render_simples(&mut self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        let block = Block::default()
            .style(Style::default().fg(Color::Reset))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        // TODO: Get title
        let title = Paragraph::new(format!(" {}", self.get_title()))
            .block(block.clone().title(" File "))
            .style(Style::default().fg(Color::Yellow));

        f.render_widget(title, layout[0]);
        f.render_widget(self.timestamp.widget(), layout[1]);
        f.render_widget(self.start.widget(), layout[2]);
        f.render_widget(self.end.widget(), layout[3]);
        f.render_widget(self.title.widget(), layout[4]);
        f.render_widget(self.language.widget(), layout[5]);
    }
    fn render_text(&mut self, f: &mut Frame, area: Rect) {
        f.render_widget(self.description.widget(), area);
    }

    fn render_lower(&mut self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        self.render_bar(f, layout[0]);
        self.render_list(f, layout[1]);
    }

    fn render_list(&mut self, f: &mut Frame, area: Rect) {
        let itms: Vec<ListItem> = self
            .tagslist
            .items()
            .iter()
            .map(|x| ListItem::new(x.to_string()).style(Style::default()))
            .collect();

        let list = List::new(itms)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .highlight_style(Style::default().fg(Color::Cyan));

        f.render_stateful_widget(list, area, &mut self.tagslist.state);
    }

    fn render_bar(&mut self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(7), Constraint::Min(0)])
            .direction(Direction::Horizontal)
            .split(area);

        let para = Paragraph::new(" Tags: ").style(Style::default()).block(
            Block::default()
                .style(Style::default())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

        f.render_widget(para, layout[0]);
        f.render_widget(self.tags.widget(), layout[1]);
    }

    fn render_meta(&mut self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Magenta))
            .title(Title::from(" Metadata ").alignment(Alignment::Center));

        // TODO: Error handling

        let para = {
            if let Some(m) = self.get_meta() {
                Paragraph::new(m.to_string())
                    .block(block)
                    .style(Style::default())
            } else {
                Paragraph::new("NO FILE SELECTED").block(block)
                // Paragraph::new(format!("{:?}", self.files.meta))
            }
        };

        f.render_widget(para, area);
    }
}

use crate::application::{load, parse};

impl Default for Import {
    fn default() -> Self {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let style = Style::default();

        let mut timestamp = TextArea::default();
        timestamp.set_placeholder_text("%d-%m-%Y");
        timestamp.set_block(block.clone());
        timestamp.set_cursor_line_style(style.clone());
        let mut start = TextArea::default();
        start.set_placeholder_text(" Start HH:MM:SS");
        start.set_block(block.clone().title(" Start Time "));
        start.set_cursor_line_style(style.clone());
        let mut end = TextArea::default();
        end.set_placeholder_text(" End HH:MM:SS");
        end.set_block(block.clone().title(" End Time "));
        end.set_cursor_line_style(style.clone());
        let mut title = TextArea::default();
        title.set_placeholder_text(" Enter Title");
        title.set_block(block.clone().title(" Enter Title "));
        title.set_cursor_line_style(style.clone());
        let mut description = TextArea::default();
        description.set_placeholder_text(" Enter Description");
        description.set_block(block.clone().title(" Description "));
        description.set_cursor_line_style(style.clone());
        let mut language = TextArea::default();
        language.set_placeholder_text(" de, en");
        language.set_block(block.clone().title(" Language "));
        language.set_cursor_line_style(style.clone());
        let mut tags = TextArea::default();
        tags.set_placeholder_text(" Enter tags");
        tags.set_block(block.clone());
        tags.set_cursor_line_style(style.clone());

        // TODO: Optimize with capacity
        let mut alltags = Vec::new();
        let tagquery = DATABASE.get_all::<Tag>().unwrap();
        for tag in tagquery {
            if let Ok(t) = tag {
                alltags.push(t);
            }
        }

        Self {
            selected: 0,
            start,
            end,
            title,
            description,
            tags,
            language,
            files: ItemList::new(load()),
            meta: Meta::default(),
            timestamp,
            tagslist: ItemList::new(alltags),
        }
    }
}
