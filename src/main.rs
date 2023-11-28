use std::time::{Duration, Instant};
use std::env;

use crossterm::{
    execute,
    cursor::{self, MoveTo},
    event::{self, poll, read, Event, KeyCode, KeyEvent, KeyEventKind},
    event::{
        DisableBracketedPaste, DisableFocusChange, EnableBracketedPaste,
        EnableFocusChange, PopKeyboardEnhancementFlags, DisableMouseCapture, EnableMouseCapture, 
    },
};

pub mod game;
use game::{board::Board, renderer};

fn main()-> std::io::Result<()> {
    let mut board = Board::new(16, 16, 20);

    renderer::initialize()?;
    
    let timer = Instant::now();
    let mut redraw_board = true;

    // Main loop
    loop {
        // This is the main game loop
        // First we check for things that would make the board be redrawn,
        // such as the timer updating or an action happening
        // And then we redraw and start again

        // If the timer needs updating
        if board.timer != timer.elapsed().as_secs() {
            board.timer = timer.elapsed().as_secs();
            redraw_board = true;
        }
        // Poll for events so it runs at just over 60fps
        if poll(Duration::from_millis(16))? { 
            let event = read()?;
            match event {
                // If a key is pressed, handle it
                Event::Key(KeyEvent { code, modifiers, kind, state }) => {
                    // Only check it if it's a Press
                    if kind != KeyEventKind::Press { continue; }
                    match code {
                        // Exiting
                        KeyCode::Esc   => { break; }
                        // Moving the cursor
                        KeyCode::Right => { board.move_selected_cell( 1, 0); redraw_board = true; }
                        KeyCode::Left  => { board.move_selected_cell(-1, 0); redraw_board = true; }
                        KeyCode::Up    => { board.move_selected_cell( 0,-1); redraw_board = true; }
                        KeyCode::Down  => { board.move_selected_cell( 0, 1); redraw_board = true; }
                        // Dig
                        KeyCode::Char(' ') | KeyCode::Enter => { board.dig(); redraw_board = true; }
                        // Flag
                        KeyCode::Char('f')  => { board.flag(); redraw_board = true; }
                        _ => {}
                    }
                },
                // If the terminal is resized, redraw!!
                Event::Resize(w, h) => {
                    renderer::clear()?;
                    redraw_board = true;
                }
                _ => { },
            }    
        }
        // Redraw the board
        if redraw_board { renderer::draw_board(&board)?; redraw_board = false; }
        // Exit if the game is over
        if board.exit.is_some() { break; }
    }
    // Clean up
    renderer::finalize()
}