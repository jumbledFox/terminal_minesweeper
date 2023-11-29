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
use game::board;
use game::{board::Board, renderer};

fn main()-> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
    
    let mut board: Option<Board> = None;
    match args.len() {
        // Normal game
        2 => match args[1].to_lowercase().as_str() {
            "easy"   => { board = Some(Board::new(board::BoardType::Easy)) }
            "normal" => { board = Some(Board::new(board::BoardType::Normal)) }
            "hard"   => { board = Some(Board::new(board::BoardType::Hard)) }
            _ => ()
        }
        // Custom game
        4 => {
            let mut params: [u16; 3] = [0; 3];
            for i in 1..=3 {
                if let Ok(param) = args[i].parse::<u16>() {
                    params[i-1] = param;
                } else {
                    println!("Argument {:?} is invalid!! must be an integer", i);
                    return Ok(());
                }
            }
            board = Some(Board::new(board::BoardType::Custom(params[0], params[1], params[2])));
        }
        _ => ()
    }
    if board.is_none() {
        println!("help text");
        return Ok(());
    }
    let mut board = board.unwrap();
    //board = Board::new(16, 16, 20);
    let timer = Instant::now();
    let mut redraw_board = true;

    renderer::initialize()?;

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
                Event::Resize(w, h) => { renderer::clear()?; redraw_board = true; }
                _ => { },
            }    
        }
        // Redraw the board
        if redraw_board { renderer::draw_screen(&board)?; redraw_board = false; }
        // Exit if the game is over
        if board.exit.is_some() { break; }
    }
    // Clean up
    renderer::finalize()
}