use std::{fs, io, path::PathBuf, process::Command};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use tui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem},
};

fn get_videos(playlist_path: &PathBuf) -> Vec<String> {
    fs::read_dir(playlist_path)
        .unwrap_or_else(|_| fs::read_dir(".").unwrap())
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Load playlists from ~/Videos/Collection
    let base_path = dirs::video_dir()
        .unwrap_or(PathBuf::from("."))
        .join("Collection");
    let playlists: Vec<_> = fs::read_dir(&base_path)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    let mut selected_playlist = 0;
    let mut selected_video = 0;
    let mut focus_on_playlists = true;

    loop {
        // Load videos in the selected playlist
        let playlist_path = base_path.join(&playlists[selected_playlist]);
        let videos = get_videos(&playlist_path);

        // UI rendering
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(f.size());

            let playlist_items: Vec<ListItem> = playlists
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    let style = if i == selected_playlist && focus_on_playlists {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    ListItem::new(Spans::from(name.clone())).style(style)
                })
                .collect();

            let video_items: Vec<ListItem> = videos
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    let style = if i == selected_video && !focus_on_playlists {
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    ListItem::new(Spans::from(name.clone())).style(style)
                })
                .collect();

            let playlist_list = List::new(playlist_items)
                .block(Block::default().borders(Borders::ALL).title("Playlists"));

            let video_list = List::new(video_items)
                .block(Block::default().borders(Borders::ALL).title("Videos"));

            f.render_widget(playlist_list, chunks[0]);
            f.render_widget(video_list, chunks[1]);
        })?;

        // Handle input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Left | KeyCode::Right => {
                    focus_on_playlists = !focus_on_playlists;
                }
                KeyCode::Up => {
                    if focus_on_playlists && selected_playlist > 0 {
                        selected_playlist -= 1;
                        selected_video = 0;
                    } else if !focus_on_playlists && selected_video > 0 {
                        selected_video -= 1;
                    }
                }
                KeyCode::Down => {
                    if focus_on_playlists {
                        if selected_playlist < playlists.len().saturating_sub(1) {
                            selected_playlist += 1;
                            selected_video = 0;
                        }
                    } else {
                        if selected_video < videos.len().saturating_sub(1) {
                            selected_video += 1;
                        }
                    }
                }

                KeyCode::Enter => {
                    if !focus_on_playlists {
                        if let Some(video) = videos.get(selected_video) {
                            let video_path = playlist_path.join(video);

                            // Run mpv in the background without blocking the terminal UI
                            let _ = Command::new("mpv")
                                .arg("--really-quiet")
                                .arg("--no-terminal")
                                .arg("--osd-level=0")
                                .arg(video_path)
                                .spawn()
                                .expect("Failed to play video");

                            // Immediately return to the UI
                            terminal.draw(|f| {
                                let chunks = Layout::default()
                                    .direction(Direction::Horizontal)
                                    .constraints([
                                        Constraint::Percentage(30),
                                        Constraint::Percentage(70),
                                    ])
                                    .split(f.size());

                                let playlist_items: Vec<ListItem> = playlists
                                    .iter()
                                    .enumerate()
                                    .map(|(i, name)| {
                                        let style = if i == selected_playlist && focus_on_playlists
                                        {
                                            Style::default()
                                                .fg(Color::Yellow)
                                                .add_modifier(Modifier::BOLD)
                                        } else {
                                            Style::default()
                                        };
                                        ListItem::new(Spans::from(name.clone())).style(style)
                                    })
                                    .collect();

                                let video_items: Vec<ListItem> = videos
                                    .iter()
                                    .enumerate()
                                    .map(|(i, name)| {
                                        let style = if i == selected_video && !focus_on_playlists {
                                            Style::default()
                                                .fg(Color::Green)
                                                .add_modifier(Modifier::BOLD)
                                        } else {
                                            Style::default()
                                        };
                                        ListItem::new(Spans::from(name.clone())).style(style)
                                    })
                                    .collect();

                                let playlist_list = List::new(playlist_items).block(
                                    Block::default().borders(Borders::ALL).title("Playlists"),
                                );

                                let video_list = List::new(video_items)
                                    .block(Block::default().borders(Borders::ALL).title("Videos"));

                                f.render_widget(playlist_list, chunks[0]);
                                f.render_widget(video_list, chunks[1]);
                            })?;
                        }
                    }
                }

                _ => {}
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
