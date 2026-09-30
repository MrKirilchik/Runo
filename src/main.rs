use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use std::{
    error::Error,
    io::stdout,
    fs,
};

mod cli;
use cli::Cli;
use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {

    let cli = Cli::parse();

    let file_path  = &cli.file.into_os_string().into_string().unwrap();

    let mut file_contents = String::new();

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|frame| {
            let text = Paragraph::new(file_contents.as_str())
                .block(
                    Block::default()
                        .borders(Borders::NONE)
                );
            
            frame.render_widget(text, frame.area());
        })?;
        
        if let Event::Key(key) = event::read()? {
            match key.code {
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        fs::write(file_path, &file_contents)?; 
                    }

                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        break; 
                    }

                    KeyCode::Backspace => {
                        file_contents.pop();
                    }
                    KeyCode::Enter => {
                        file_contents.push_str("\n");
                    }

                    KeyCode::Char(c) => {
                        file_contents.push(c); // Просто добавляем нажатый символ в конец строки
                    }

                    _ => {}
                }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}