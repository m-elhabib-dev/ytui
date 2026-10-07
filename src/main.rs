use anyhow::Context;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::Paragraph,
};

fn main() -> anyhow::Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> anyhow::Result<()> {
    loop {
        terminal
            .draw(|frame| frame.render_widget(Paragraph::new("Some Text!"), frame.area()))
            .context("failed in draw in the terminal")?;
        let event = event::read().context("couldnt read event")?;
        if let Event::Key(key) = event
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') => {
                    break;
                }
                _ => (),
            }
        }
    }
    Ok(())
}
