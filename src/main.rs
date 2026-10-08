use std::process::{Child, Command, Stdio};

use anyhow::Context;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::Paragraph,
};

enum Mode {
    Editing {
        input: String,
    },
    Downloading {
        url: String,
        child: Child,
        percent: f64,
    },
}

struct App {
    mode: Mode,
    quit: bool,
}

fn main() -> anyhow::Result<()> {
    let terminal = ratatui::init();
    let app = App {
        quit: false,
        mode: Mode::Editing {
            input: String::new(),
        },
    };
    let result = run(terminal, app);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, mut app: App) -> anyhow::Result<()> {
    loop {
        let text = match &mut app.mode {
            Mode::Editing { input } => input,
            Mode::Downloading { .. } => "Downloading Mode",
        };
        terminal
            .draw(|frame| frame.render_widget(Paragraph::new(text), frame.area()))
            .context("failed in draw in the terminal")?;
        let event = event::read().context("couldnt read event")?;
        if let Event::Key(key) = event
            && key.kind == KeyEventKind::Press
        {
            match &mut app.mode {
                Mode::Editing { input } => match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        app.mode = Mode::Downloading {
                            url: std::mem::take(input),
                            child: Command::new("ls")
                                .arg("-lh")
                                .stdout(Stdio::piped())
                                .stderr(Stdio::piped())
                                .spawn()?,
                            percent: 0_f64,
                        }
                    }
                    _ => (),
                },
                Mode::Downloading { .. } => match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Char('q') => break,
                    _ => (),
                },
            }
        }
    }
    Ok(())
}
