use std::{io::{stdout, Write}, time::Duration};

use crossterm::{
    execute,
    cursor,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize},
    ExecutableCommand,
    event::{self, poll, read, Event, KeyCode, KeyEvent, KeyEventState, KeyEventKind},
    event::{
        DisableBracketedPaste, DisableFocusChange, EnableBracketedPaste,
        EnableFocusChange, PopKeyboardEnhancementFlags, DisableMouseCapture, EnableMouseCapture, 
    },
    terminal::{enable_raw_mode, disable_raw_mode},
};

pub mod game;
use game::{board::Board, renderer};

fn main()-> std::io::Result<()> {
    enable_raw_mode()?;
    let mut board = Board::new(30, 16, 99);
    execute!(
        stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        ResetColor,
        cursor::MoveTo(0, 0),
    )?;

    // Main loop
    loop {
        // Draw the board
        renderer::draw_board(&board)?;

        let event = read()?;

        match event {
            Event::Key(KeyEvent { code, modifiers, kind, state }) => {
                if kind == KeyEventKind::Press {
                    match code {
                        // Exiting
                        KeyCode::Esc   => { break; }
                        // Moving the cursor
                        KeyCode::Right => { board.move_selected_cell( 1, 0); continue; }
                        KeyCode::Left  => { board.move_selected_cell(-1, 0); continue; }
                        KeyCode::Up    => { board.move_selected_cell( 0,-1); continue; }
                        KeyCode::Down  => { board.move_selected_cell( 0, 1); continue; }
                        // Dig
                        KeyCode::Char(' ') | KeyCode::Enter => { board.dig(); continue; }
                        // Flag
                        KeyCode::Char('f')  => { board.flag(); continue; }
                        _ => {}
                    }
                }
                
            },
            _ => {},
        }
        poll(Duration::from_millis(1_000))?;
    }
    
    // Clean up
    execute!(
        stdout(),
        DisableBracketedPaste,
        PopKeyboardEnhancementFlags,
        DisableFocusChange,
        DisableMouseCapture,
    )?;

    disable_raw_mode()
}