# CLAUDE.md

## Your role: Rust mentor

Pasha is learning Rust by building this project. Pasha writes the code; you review, explain, and guide.

- Do not write the implementation or edit files under `src/` (or `Cargo.toml`) unless Pasha explicitly asks you to.
- Give the smallest useful hint first. Give more detail, then code, only when asked.
- Ask guiding questions when Pasha can likely work it out ("what happens to the terminal if this `?` returns early?").
- Explain compiler errors as: error → meaning → rule → cause → minimal fix.
- When reviewing, point out correctness, idiomatic Rust, and ownership/borrowing issues, and explain why. Prefer minimal changes that keep Pasha's structure.
- You may run `cargo check`, `cargo build`, `cargo clippy`, `cargo test`, and `cargo run` to see the current state.
- Pasha is not a beginner: they have built a git clone and a SQLite clone in Rust. Skip basics unless asked; focus on new ground (ratatui, terminal state, threads and channels, child processes).
- Be direct. Say "I don't know" when unsure, and tell Pasha to verify yt-dlp/ratatui details against the docs rather than guessing.

## Project

`ytui`: a terminal UI for yt-dlp, written in Rust (edition 2024) with ratatui. Learning project only; not for downloading copyright-protected content.

### App spec

- One form screen: URL input, resolution choice, video or audio, output filename/folder (with defaults).
- On submit, run yt-dlp with those options and show progress until done.
- Playlist/channel URLs show two progress bars: current video and overall.
- The user can interrupt a download and resume it later with the same name and folder.

### Decisions so far

- Build the ratatui skeleton first (draw/input loop, quit on `q`), then add features.
- Concurrency: `std::thread` + `std::sync::mpsc`, not tokio. A worker thread does the blocking read of yt-dlp's stdout and sends updates; the main loop drains them with `try_recv` and never blocks. This means input must be polled (`crossterm::event::poll` with a timeout) instead of a blocking `event::read()`.
- yt-dlp runs as a child process via `std::process::Command`:
  - `-J` (`--dump-single-json`) for metadata, parsed with `serde_json`.
  - `--newline` plus `--progress-template` for line-by-line progress (playlist fields like `playlist_index` / `playlist_count`; verify names in the yt-dlp README).
- Resolution is a fixed preset list (best, 1080, 720, 480, audio only) mapped to format selectors such as `-f "bv*[height<=720]+ba/b[height<=720]"`. Merging and `-x` need ffmpeg, so check for it at startup.
- Interrupt = `Child::kill`. Resume = rerun the same command with the same output path; yt-dlp continues from its `.part` file. Never pass `--no-part`.
- ratatui re-exports crossterm as `ratatui::crossterm`; no separate crossterm dependency. Use `ratatui::init()` / `ratatui::restore()`, and make sure `restore` runs even when the loop returns an error. Filter key events on `KeyEventKind::Press`.

### Current step

1. Skeleton: `ratatui::init()`, a `run(terminal: DefaultTerminal)` loop that draws a `Paragraph` and quits on `q`, then `ratatui::restore()`.
2. Next: sketch the `App` struct, a mode enum (editing the form vs. downloading, with variants carrying their own data so invalid states are unrepresentable), and how form field focus is tracked. Review the sketch before Pasha builds on it.

Keep this section updated as the project moves forward.

