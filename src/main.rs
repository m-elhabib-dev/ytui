use std::{
    io::{self, BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::mpsc,
    thread,
};

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
        rx: mpsc::Receiver<Update>,
    },
}

struct App {
    mode: Mode,
    quit: bool,
}

enum Update {
    Progress { percent: f64 },
    Output { text: String },
    Finished,
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
                        let mut child = Command::new("ls")
                            .arg("-lh")
                            .stdout(Stdio::piped())
                            .stderr(Stdio::null())
                            .spawn()?;
                        let child_stdout =
                            child.stdout.take().expect("the stdout must not be empty");
                        let reader = BufReader::new(child_stdout);
                        let (tx, rx) = mpsc::channel();
                        thread::spawn(move || {
                            for line in reader.lines() {
                                let Ok(line) = line else {
                                    break;
                                };
                                if tx.send(Update::Output { text: line }).is_err() {
                                    break;
                                };
                                rx.try_recv();
                            }
                            let _ = tx.send(Update::Finished);
                        });
                        app.mode = Mode::Downloading {
                            url: std::mem::take(input),
                            child,
                            percent: 0_f64,
                            rx,
                        };
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
