use std::io::{stdout, Write};

use crossterm::{
    execute,
    cursor,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize},
    ExecutableCommand,
    event, terminal::enable_raw_mode,
};

pub mod game;
use game::{board::Board, renderer};

fn main()-> std::io::Result<()> {
    enable_raw_mode()?;
    let board = Board::new(11, 11, 10);
    execute!(
        stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        ResetColor,
        cursor::MoveTo(0, 0),
    )?;
    let mut selected_cell: [u16; 2] = [0, 0];
    // Draw the board
    renderer::draw_board(&board, &selected_cell)?;
    

    Ok(())
}