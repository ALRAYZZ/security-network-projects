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
    widgets::{Block, Borders, Paragraph},
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

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ]
                        .as_ref(),
                )
                .split(size);

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

            f.render_widget(download, chunks[0]);
            f.render_widget(upload, chunks[1]);
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
