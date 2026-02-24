use std::{io, thread};
use std::time::Duration;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Sparkline},
    Terminal,
};

mod engine;
use engine::{NetworkEngine, human_readable};


fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut engine = NetworkEngine::new("Ethernet 3");

    loop {
        let stats = engine.update();

        terminal.draw(|f| {
            let size = f.size();

            // Creates the layout of the terminal
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(5),
                        Constraint::Length(5),
                    ]
                        .as_ref(),
                )
                .split(size);

            // Create the widgets for download and upload speeds
           let download = Paragraph::new(format!(
               "Download: {}",
               human_readable(stats.download_bps)
           ))
               .block(Block::default().borders(Borders::ALL).title("Download"));

            let upload = Paragraph::new(format!(
                "Upload: {}",
                human_readable(stats.upload_bps)
            ))
                .block(Block::default().borders(Borders::ALL).title("Upload"));

            // Scale the history data for the sparkline graphs since data method just takes u64
            let download_scaled: Vec<u64> = stats
                .download_history
                .iter()
                .map(|x| *x as u64)
                .collect();
            let upload_scaled: Vec<u64> = stats
                .upload_history
                .iter()
                .map(|x| *x as u64)
                .collect();

            let download_graph = Sparkline::default()
                .block(Block::default().borders(Borders::ALL).title("Download History"))
                .data(&download_scaled);

            let upload_graph = Sparkline::default()
                .block(Block::default().borders(Borders::ALL).title("Upload History"))
                .data(&upload_scaled);

            f.render_widget(download, chunks[0]);
            f.render_widget(upload, chunks[1]);
            f.render_widget(download_graph, chunks[2]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
