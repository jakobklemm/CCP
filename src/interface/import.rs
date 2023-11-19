//! # Import

use std::{
    fs::File,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use crate::{
    entry::Id,
    interface::{Render, TextArea},
    metadata::{self, Metadata},
    processor::{Job, Language, Timestamp},
    update::control,
    util, DATABASE, ROOT,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::Rect,
    style::{Color, Style},
    widgets::{block::Title, Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

const INPUTS: usize = 5;

/// TODO: Add Tags list and file tree.
#[derive(Debug, Clone)]
pub struct Import {
    current: usize,
    start: TextArea,
    end: TextArea,
    title: TextArea,
    selector: TextArea,
    language: TextArea,
    files: FileList,
    popped: bool,
    valid: bool,
    id: String,
    moment: Instant,
}

#[derive(Clone, Debug)]
struct FileList {
    state: ListState,
    items: Vec<String>,
    meta: Vec<Metadata>,
}

use std::fs;

impl FileList {
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (self.items.len() + i - 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn load(&mut self) {
        self.items.clear();
        self.meta.clear();
        let path = format!("{}/ingest/", ROOT.as_str());
        if let Ok(itr) = fs::read_dir(path.clone()) {
            for entry in itr.into_iter() {
                if entry.is_err() {
                    continue;
                }
                let entry = entry.unwrap();
                let name = entry.file_name();
                if let Ok(i) = name.into_string() {
                    let m = Metadata::new(format!("{}{}", path, i));
                    self.items.push(i);
                    self.meta.push(m);
                }
            }
        }
    }
}

impl Default for FileList {
    fn default() -> Self {
        let mut list = FileList {
            state: ListState::default(),
            items: Vec::new(),
            meta: Vec::new(),
        };

        list.load();

        list
    }
}

impl Render for Import {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        if self.popped {
            if self.moment.elapsed().as_secs_f64() > 1.25 {
                self.popped = false;
            } else {
                let (text, style) = if self.valid {
                    (
                        format!("Job {} created!", self.id),
                        Style::default().fg(Color::Green),
                    )
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

        match self.current {
            0 => {
                self.reset_all();
                self.start.set_block(accept.title(" Start Time "));
            }
            1 => {
                self.reset_all();
                self.end.set_block(accept.title(" End Time "));
            }
            2 => {
                self.reset_all();
                self.language.set_block(accept.title(" Language "));
            }
            3 => {
                self.reset_all();
                self.title.set_block(accept.title(" Enter Title "));
            }
            4 => {
                self.reset_all();
                self.selector.set_block(accept.title(" Enter Tags "));
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
            KeyCode::Enter => {
                self.focus_next();
            }
            KeyCode::Down if control(&key) => {
                self.focus_next();
            }
            KeyCode::Up if control(&key) => {
                self.focus_prev();
            }
            KeyCode::Char('s') if key.modifiers == KeyModifiers::CONTROL => {
                self.save_job();
            }
            KeyCode::Char('j') if key.modifiers == KeyModifiers::CONTROL => {
                self.files.next();
            }
            KeyCode::Char('k') if key.modifiers == KeyModifiers::CONTROL => {
                self.files.previous();
            }
            _ => {
                match self.current {
                    0 => self.start.input(key),
                    1 => self.end.input(key),
                    2 => self.language.input(key),
                    3 => self.title.input(key),
                    4 => self.selector.input(key),
                    _ => false,
                };
            }
        }
    }
}

impl Import {
    fn save_job(&mut self) {
        self.id = String::new();
        self.popped = true;
        self.moment = Instant::now();
        let start = Timestamp::from_input(self.start.lines());
        let end = Timestamp::from_input(self.end.lines());
        let title = Timestamp::from_input(self.title.lines());
        let lang = Language::from_input(self.language.lines());

        // 0. Check if file selected
        let file = {
            if let Some(ind) = self.get_path() {
                ind
            } else {
                self.valid = false;
                return;
            }
        };
        // 1. Create job
        let mut job = {
            // TODO: Add tags
            if let Some(j) = Job::new(start, end, file, title, Vec::new()) {
                self.valid = true;
                j
            } else {
                self.valid = false;
                return;
            }
        };
        job.set_language(lang);
        self.id = job.key.to_string();
        // 2. Check validity
        if !self.valid {
            return;
        }
        // 3. Submit to DB
        let col = DATABASE.collection::<Job>("jobs");
        if let Err(_res) = col.insert_one(job) {
            self.valid = false;
            return;
        }
        // 4. Reset inputs
        let def = Self::default();
        self.language = def.language;
        self.start = def.start;
        self.end = def.end;
        self.title = def.title;
        self.selector = def.selector;
        // 5. Popup
        self.popped = true;
        self.moment = Instant::now();
    }

    fn reset_all(&mut self) {
        let block = Block::default()
            .style(Style::default().fg(Color::Gray))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        self.start.set_block(block.clone().title(" Start Time "));
        self.end.set_block(block.clone().title(" End Time "));
        self.title.set_block(block.clone().title(" Enter Title "));
        self.language.set_block(block.clone().title(" Language "));
        self.selector.set_block(block.clone().title(" Enter Tags "));
    }

    fn focus_next(&mut self) {
        self.current = (self.current + 1) % INPUTS;
    }

    fn focus_prev(&mut self) {
        self.current = (INPUTS + self.current - 1) % INPUTS;
    }

    fn render_tree(&mut self, f: &mut Frame, area: Rect) {
        let itms: Vec<ListItem> = self
            .files
            .items
            .iter()
            .map(|x| {
                let t = format!("{}", x);
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
        if let Some(i) = self.files.state.selected() {
            if let Some(s) = self.files.items.get(i) {
                s.to_string()
            } else {
                String::from("NO FILE SELECTED")
            }
        } else {
            String::from("NO FILE SELECED")
        }
    }

    fn get_meta(&self) -> Option<Metadata> {
        if let Some(i) = self.files.state.selected() {
            self.files.meta.get(i).cloned()
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

        let identity = Paragraph::new(format!(" {}", 42))
            .block(block.clone().title(" Entity ID "))
            .style(Style::default().fg(Color::DarkGray));

        let name = Paragraph::new(format!(" test.mp4"))
            .block(block.clone().title(" File Name "))
            .style(Style::default().fg(Color::DarkGray));

        let ts = Paragraph::new(format!(
            " {:?}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ))
        .block(block.clone().title(" Timestamp "))
        .style(Style::default().fg(Color::DarkGray));

        f.render_widget(title, layout[0]);
        f.render_widget(identity, layout[1]);
        f.render_widget(name, layout[2]);
        f.render_widget(ts, layout[3]);
        f.render_widget(self.start.widget(), layout[4]);
        f.render_widget(self.end.widget(), layout[5]);
        f.render_widget(self.language.widget(), layout[6]);
    }
    fn render_text(&mut self, f: &mut Frame, area: Rect) {
        f.render_widget(self.title.widget(), area);
    }

    fn render_lower(&mut self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        self.render_bar(f, layout[0]);
        self.render_list(f, layout[1]);
    }

    fn render_list(&mut self, f: &mut Frame, area: Rect) {
        f.render_widget(
            Paragraph::new("TODO: Tags list").block(Block::default().borders(Borders::ALL)),
            area,
        );
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
        f.render_widget(self.selector.widget(), layout[1]);
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

impl Default for Import {
    fn default() -> Self {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let mut start = TextArea::default();
        start.set_placeholder_text(" Start HH:MM:SS");
        start.set_block(block.clone().title(" Start Time "));
        let mut end = TextArea::default();
        end.set_placeholder_text(" End HH:MM:SS");
        end.set_block(block.clone().title(" End Time "));
        let mut title = TextArea::default();
        title.set_placeholder_text(" Enter Title");
        title.set_block(block.clone().title(" Enter Title "));
        let mut language = TextArea::default();
        language.set_placeholder_text(" de, en");
        language.set_block(block.clone().title(" Language "));
        let mut selector = TextArea::default();
        selector.set_placeholder_text(" Enter tags");
        selector.set_block(block.clone());

        Self {
            id: String::new(),
            current: 0,
            start,
            end,
            title,
            selector,
            language,
            files: FileList::default(),
            popped: false,
            valid: false,
            moment: Instant::now(),
        }
    }
}
