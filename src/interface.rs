//! Interface

use crate::application::App;

use ratatui::{
    prelude::{Alignment, Frame, CrosstermBackend},
    style::{Color, Style},
    backend::Backend,
    widgets::{Block, BorderType, Borders, Paragraph},
};


pub fn render<B: Backend>(app: &mut App, f: &mut Frame<B>) {
    f.render_widget(
        Paragraph::new(format!(
            "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
        Counter: {}
      ",
            app.get_counter()
        ))
        .block(
            Block::default()
                .title(" TAB Central Content Processor ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        f.size(),
    )
}
