//! # Execute

use std::sync::mpsc::Receiver;

use crate::application::processor;
use crate::application::status::Status;
use crate::interface::list::ItemList;
use crate::DATABASE;
use crate::{application::job::Job, interface::Render};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use polodb_core::bson::doc;
use ratatui::style::{Modifier, Stylize};
use ratatui::widgets::Gauge;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

#[derive(Debug)]
pub struct Execute {
    count: usize,
    current: Option<Job>,
    receiver: Option<Receiver<Status>>,
    status: Status,
    list: ItemList<Job>,
}

impl Clone for Execute {
    fn clone(&self) -> Self {
        Self {
            count: self.count + 1,
            current: self.current.clone(),
            receiver: None,
            status: self.status.clone(),
            list: self.list.clone(),
        }
    }
}

impl Default for Execute {
    fn default() -> Self {
        let mut list = ItemList::default();
        let mut jobs: Vec<Job> = Vec::new();
        let found = DATABASE.get_many(doc! {"done": false}).unwrap();
        for job in found {
            if let Ok(j) = job {
                jobs.push(j);
            }
        }
        list.set(jobs);
        Self {
            count: 0,
            list,
            current: None,
            receiver: None,
            status: Default::default(),
        }
    }
}

impl Render for Execute {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        // TODO: Does this work?
        if let Some(recv) = &self.receiver {
            if let Ok(s) = recv.try_recv() {
                self.status = s;
            }
        }

        match self.status {
            Status::Complete(_) => {
                self.current = None;
                self.receiver = None;
                let mut list = ItemList::default();
                let mut jobs: Vec<Job> = Vec::new();
                let found = DATABASE.get_many(doc! {"done": false}).unwrap();
                for job in found {
                    if let Ok(j) = job {
                        jobs.push(j);
                    }
                }
                list.set(jobs);
                self.list = list;
            }
            _ => {}
        }

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(10), Constraint::Min(0)])
            .split(area);

        self.render_bar(f, layout[0]);
        self.render_list(f, layout[1]);
    }

    fn input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('j') if key.modifiers == KeyModifiers::CONTROL => {
                self.list.next();
            }
            KeyCode::Char('k') if key.modifiers == KeyModifiers::CONTROL => {
                self.list.previous();
            }
            KeyCode::Enter => {
                // Execute current job
                self.current = self.list.get();
                if let Some(job) = &self.current {
                    self.receiver = Some(processor::execute(job.clone()));
                }
            }
            _ => {}
        }
    }
}

impl Execute {
    fn render_bar(&mut self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(area);

        let text = {
            if let Some(j) = &self.current {
                format!("Execution: {}", j.get_file())
            } else {
                String::from("Execution: ")
            }
        };

        f.render_widget(
            Paragraph::new(text).style(Style::default().bold()),
            layout[0],
        );
        self.render_parts(f, layout[1]);
    }

    fn render_parts(&mut self, f: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(area);

        self.render_part_one(f, layout[0]);
        self.render_part_two(f, layout[1]);
        self.render_part_three(f, layout[2]);
    }

    fn render_part_one(&mut self, f: &mut Frame, area: Rect) {
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title(" Part 1 "))
            .gauge_style(
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .use_unicode(true)
            .percent(self.status.get_perc().0);

        f.render_widget(gauge, area);
    }

    fn render_part_two(&mut self, f: &mut Frame, area: Rect) {
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title(" Part 2 "))
            .gauge_style(
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .use_unicode(true)
            .percent(self.status.get_perc().1);

        f.render_widget(gauge, area);
    }

    fn render_part_three(&mut self, f: &mut Frame, area: Rect) {
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title(" Part 3 "))
            .gauge_style(
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .use_unicode(true)
            .percent(self.status.get_perc().2);

        f.render_widget(gauge, area);
    }

    fn render_list(&mut self, f: &mut Frame, area: Rect) {
        let listed: Vec<ListItem> = self
            .list
            .items()
            .iter()
            .map(|x| ListItem::new(x.to_string()))
            .collect();

        let list = List::new(listed)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .highlight_style(Style::default().fg(Color::LightRed));

        f.render_stateful_widget(list, area, &mut self.list.state);
    }
}
