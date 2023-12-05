//! # Execute

use crate::interface::Render;
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use super::TextArea;

#[derive(Clone, Debug)]
pub struct Execute {
    input: TextArea,
    list: JobList,
}

impl Default for Execute {
    fn default() -> Self {
        let mut list = JobList::default();
        list.fetch();
        Self {
            input: TextArea::default(),
            list,
        }
    }
}

impl Render for Execute {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        // let layout = Layout::default()
        //     .direction(Direction::Vertical)
        //     .constraints([Constraint::Length(3), Constraint::Min(0)])
        //     .split(area);
        //
        // self.render_bar(f, layout[0]);
        // self.render_list(f, layout[1]);
    }

    fn input(&mut self, key: KeyEvent) {}
}

impl Execute {
    fn render_bar(&mut self, f: &mut Frame, area: Rect) {
        f.render_widget(self.input.widget(), area)
    }

    fn render_list(&mut self, f: &mut Frame, area: Rect) {
        // let listed: Vec<ListItem> = self
        //     .list
        //     .items
        //     .iter()
        //     .map(|x| {
        //         let t = format!("{}", x.title);
        //         ListItem::new(t)
        //     })
        //     .collect();
        //
        // let list = List::new(listed)
        //     .block(
        //         Block::default()
        //             .borders(Borders::ALL)
        //             .border_type(BorderType::Rounded),
        //     )
        //     .highlight_style(Style::default().fg(Color::LightRed));
        //
        // f.render_stateful_widget(list, area, &mut self.list.state);
    }
}

#[derive(Clone, Debug, Default)]
struct JobList {
    // pub state: ListState,
    // pub items: Vec<Job>,
}

impl JobList {
    fn fetch(&mut self) {}
}
