use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Paragraph},
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

use ropey::Rope;

fn main() -> Result<(), Box<dyn Error>> { 

    let cli = Cli::parse();

    let file_path:          &String = &cli.file.into_os_string().into_string().unwrap();

    let mut file_contents:  String  = fs::read_to_string(file_path)?;
    let mut file_contents_rope: Rope = Rope::from_str(&file_contents);
    let mut file_contents_len: usize = file_contents_rope.len_chars();

    let mut current_pos:   usize    = file_contents_len;

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
                        if current_pos > 0 {
                            file_contents.remove(current_pos-1);
                            current_pos -= 1;
                        }
                    }
                    KeyCode::Enter => {
                        file_contents.push('\n');
                    }
                    KeyCode::Left => {
                        current_pos -= 1;
                    }

                    KeyCode::Right => {
                        if current_pos >= file_contents_len {
                            current_pos = file_contents_len;
                        }
                        else {
                            current_pos += 1;
                        }
                    }

                    KeyCode::Char(c) => {
                        file_contents.push(c);
                    }

                    _ => {}
                }

                file_contents_rope = Rope::from_str(&file_contents);
                file_contents_len  = file_contents_rope.len_chars();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}