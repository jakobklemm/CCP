//! # Settings

use ratatui::widgets::{block::Title, Block, BorderType, Borders, Paragraph};

use crate::interface::Render;

#[derive(Clone, Debug, Default)]
pub struct Settings {}

impl Render for Settings {
    fn render(&mut self, f: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
        let text = format!(
            "Clear FTS Cache \nCheck for entropy issues. \nTODO: Add buttons \nRefetch aggregates"
        );
        f.render_widget(
            Paragraph::new(text).block(
                Block::default()
                    .title(Title::from(" Settings "))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
            area,
        )
    }

    fn input(&mut self, _key: crossterm::event::KeyEvent) {}
}
