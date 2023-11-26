// Todo: rethink whatever this shitty file is

use crate::game::board::Tile;

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor, SetBackgroundColor, Print, Stylize, StyledContent, Attribute, SetAttribute},
    execute,
    cursor, ExecutableCommand,
};

use core::num;
use std::{borrow::Cow, io::stdout, cmp::Reverse};

use super::board::Board;

pub fn draw_board(board: &Board, selected_cell: &[u16; 2]) -> std::io::Result<()> {
    for y in 0..board.height {
        // Define empty string for a line
        let mut l: String = String::new();
        // For each character, add the tile to the string
        for x in 0..board.width {
            // Get the tile
            let tile = board.get_tile(x, y);
            // And turn it into a string
            // let mut tile_str: StyledContent<String> = match tile {
            //     &Tile::Blank                => " - ".to_owned().with(Color::Rgb { r: 170, g: 170, b: 170 }),
            //     &Tile::Unopened             => " - ".to_owned().on  (Color::Rgb { r: 200, g: 200, b: 200 }),
            //     &Tile::Flag                 => " F ".to_owned().on  (Color::Rgb { r: 224, g: 83,  b: 160 }),
            //     &Tile::Numbered(number) => format!(" {} ", number).with(get_number_color(number)),
            // };
            let mut tile_str: StyledContent<String> = match tile {
                &Tile::Blank                =>        " - ".to_owned(),
                &Tile::Unopened             =>        " - ".to_owned(),
                &Tile::Flag                 =>        " F ".to_owned(),
                &Tile::Numbered(number) => format!(" {} ", number),
            }.stylize();
            // if board.bombs.contains(&[x, y]) {
            //     tile_str = "!!!".to_owned().white().on(Color::Rgb{ r: 238, g: 181, b: 112 });
            // }
            // Get the style info of the tile
            let style_info = get_tile_style(tile);
            // If the tile is meant to be reversed, reverse it
            if style_info.reversed {
                tile_str = tile_str.reverse();
            }
            // Push that string to the line string
            l.push_str(&format!("{}", tile_str.with(style_info.fg).on(style_info.bg)));
        }
        // And then draw the line
        execute!(
            stdout(),
            cursor::MoveTo(0, y),
            Print(l),
        )?;
    }
    // Draw cursor
    let cursor_col = get_tile_style(&board.get_tile(selected_cell[0], selected_cell[1]));

    stdout().execute(ResetColor)?;
    if cursor_col.reversed {
        stdout().execute(SetAttribute(Attribute::Reverse))?;
    }
    execute!(
        stdout(),
        SetForegroundColor(cursor_col.fg),
        SetBackgroundColor(cursor_col.bg),
        cursor::MoveTo(selected_cell[0] * 3, selected_cell[1]),
        Print("["),
        cursor::MoveTo(selected_cell[0] * 3 + 2, selected_cell[1]),
        Print("]"),
        ResetColor,
        cursor::MoveTo(0, board.height),
    )?;
    
    Ok(())
}

struct StyleInfo {
    fg: Color,
    bg: Color,
    reversed: bool,
}

// Gets the color pair of a specified tile
fn get_tile_style(tile: &Tile) -> StyleInfo {
    match tile {
        Tile::Numbered(num) => match num {
            1 => StyleInfo { fg: Color::Rgb { r: 112, g: 181, b: 238 }, bg: Color::Reset, reversed: false }, 
            2 => StyleInfo { fg: Color::Rgb { r: 181, g: 238, b: 112 }, bg: Color::Reset, reversed: false }, 
            3 => StyleInfo { fg: Color::Rgb { r: 238, g: 112, b: 181 }, bg: Color::Reset, reversed: false }, 
            4 => StyleInfo { fg: Color::Rgb { r: 181, g: 112, b: 238 }, bg: Color::Reset, reversed: false }, 
            5 => StyleInfo { fg: Color::Rgb { r: 234, g: 186, b: 255 }, bg: Color::Reset, reversed: false }, 
            6 => StyleInfo { fg: Color::Rgb { r: 238, g: 181, b: 112 }, bg: Color::Reset, reversed: false }, 
            7 => StyleInfo { fg: Color::Rgb { r: 255, g: 236, b: 188 }, bg: Color::Reset, reversed: false }, 
            _ => StyleInfo { fg: Color::Rgb { r: 112, g: 238, b: 181 }, bg: Color::Reset, reversed: false }, 
        }
        Tile::Blank    => StyleInfo { fg: Color::Rgb { r: 170, g: 170, b: 170 }, bg: Color::Reset, reversed: false },
        Tile::Unopened => StyleInfo { fg: Color::Rgb { r: 170, g: 170, b: 170 }, bg: Color::Reset, reversed: true  },
        Tile::Flag     => StyleInfo { fg: Color::Rgb { r: 224, g: 83,  b: 160 }, bg: Color::Reset, reversed: true  },
    }
}