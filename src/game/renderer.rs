// Todo: rethink whatever this shitty file is

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor, SetBackgroundColor, Print, Stylize, StyledContent, Attribute, SetAttribute},
    execute,
    terminal::{Clear, ClearType},
    cursor, ExecutableCommand,
    terminal::{enable_raw_mode, disable_raw_mode},
};

use std::io::stdout;

use crate::game::board::{Board, Tile};

use super::board;

pub fn initialize() -> std::io::Result<()> {
    enable_raw_mode()?;
    execute!(
        stdout(),
        cursor::Hide,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        ResetColor,
        cursor::MoveTo(0, 0),
    )
}
   
// Clean up the application ready for exiting (need to do some research into this)
pub fn finalize() -> std::io::Result<()> {
    execute!(
        stdout(),
        // Go to bottom of screen
        cursor::MoveTo(0, crossterm::terminal::size().unwrap().1),
        cursor::Show,
        ResetColor,
        //DisableBracketedPaste,
        //PopKeyboardEnhancementFlags,
        //DisableFocusChange,
        //DisableMouseCapture,
    )?;
    disable_raw_mode()
}

pub fn help_screen() -> std::io::Result<()> {
    // Please... this is so ugly....
    execute!(
        stdout(),
        SetForegroundColor(get_tile_style(None).0),
        SetAttribute(Attribute::Underlined),
        Print("jumbledFox's epic console minesweeper game\n\r"),
        SetAttribute(Attribute::NoUnderline),
        ResetColor,
        Print("Run with arguments "),
        SetForegroundColor(Color::Green),
        Print("`easy`"),
        ResetColor,
        Print(", "),
        SetForegroundColor(Color::Cyan),
        Print("`normal`"),
        ResetColor,
        Print(", or "),
        SetForegroundColor(Color::Red),
        Print("`hard`"),
        ResetColor,
        Print(" for different difficulties.\n\rFor custom settings, use arguments for "),
        SetForegroundColor(Color::Yellow),
        Print("`width`"),
        ResetColor,
        Print(", "),
        SetForegroundColor(Color::Yellow),
        Print("`height`"),
        ResetColor,
        Print(", and "),
        SetForegroundColor(Color::Yellow),
        Print("`bomb_count`"),
        ResetColor,
        Print(", \n\re.g. "),
        SetForegroundColor(Color::Yellow),
        Print("`8 8 10`"),
        ResetColor,
        Print(" for a quick little game."),
    )?;
    disable_raw_mode()
}


// Clears the screen
pub fn clear() -> std::io::Result<()> {
    execute!(
        stdout(),
        cursor::MoveTo(0, 0),
        Clear(ClearType::All),
    )
}

// Draws the board, cursor, and titlebar
pub fn draw_screen(board: &Board) -> std::io::Result<()> {
    let mut lines = String::new();
    // Loop each line
    for y in 0..board.height {
        // Define empty string for line
        let mut l: String = String::new();
        // For each character, add the tile to the string
        for x in 0..board.width {
            // Get the tile
            let tile = board.get_tile(x, y);
            // And turn it into a string
            let mut tile_str: StyledContent<String> = match tile {
                &Tile::Blank                =>        " - ".to_owned(),
                &Tile::Unopened             =>        " - ".to_owned(),
                &Tile::Flag                 =>        " F ".to_owned(),
                &Tile::Numbered(number) => format!(" {} ", number),
            }.stylize();
            // Get the style info of the tile
            let mut style_info = get_tile_style(Some(tile));

            
            let bomb_at_current_pos = board.bombs.contains(&(x, y));
            // If the game is over, draw all of the flags where there WASN'T a bomb with a line through
            if board.exit.is_some() {
                // If there's not a bomb at the current position and it's a flag, show that the flag was wrong!
                if !bomb_at_current_pos && tile == &Tile::Flag {
                    tile_str = tile_str.crossed_out();
                }
            }        
            if board.exit == Some(board::ExitType::Lose) && board.bombs.contains(&(x, y)) && tile != &Tile::Flag {
                // If there's a bomb at the current position and the tile isn't a flag, show it!!
                if bomb_at_current_pos && tile != &Tile::Flag {
                    tile_str = " ! ".to_owned().stylize();
                    style_info = get_tile_style(None);
                }
            }
            // If the tile is meant to be reversed, reverse it
            if style_info.2 {
                tile_str = tile_str.reverse();
            }
            // Push that string to the line string
            l.push_str(&format!("{}", tile_str.with(style_info.0).on(style_info.1)));
        }
        // Add the line to the lines string
        lines.push_str(&l);
        lines.push_str(&format!("{}", " \n\r".stylize().reset()));
    }


    // Generate the titlebar
    // This is really really ugly.. TODO: Make better :c (maybe its own separate function)
    let mut title_bar: String = format!("{}", "jumbledFox's Minesweeper".to_owned().stylize().with(Color::Rgb {r: 239, g: 125, b: 87}));
    title_bar.push_str(&format!("{}", " - ".stylize().grey()));
    let board_type = Board::get_type_values(&board.board_type);
    title_bar.push_str(&format!("{} ", board_type.3.stylize().with(board_type.4)));
    title_bar.push_str(&format!("{}", &format!("({}x{}, {} mines)", board_type.0, board_type.1, board_type.2).stylize().with(board_type.5)));

    title_bar.push_str("\n\r");
    title_bar.push_str(&format!("{}", "Mines: "));
    let mines_left = match board.flag_count >= board.bomb_count as usize {
        false => board.bomb_count as usize - board.flag_count,
        _ => 0,
    };
    title_bar.push_str(&format!("{}", &format!("{:0>2}", mines_left.to_string()).stylize().red()));
    title_bar.push_str(&format!("{}", "    Time: "));
    title_bar.push_str(&format!("{}", &format!("{:0>3}", board.timer.to_string()).stylize().red()));
    title_bar.push_str("\n\r");
    title_bar.push_str(&lines);
    if board.exit.is_some() {
        let s = match &board.exit.unwrap() {
            board::ExitType::Win  => "You win, well done!",
            board::ExitType::Lose => "BANG! You lose, better luck next time!",
        };
        title_bar.push_str(s);
    }
    // Draw the screen
    execute!(
        stdout(),
        cursor::MoveTo(0, 0),
        Print(title_bar),
    )?;

    // Draw cursor
    let cursor_col = get_tile_style(Some(&board.get_tile(board.selected_cell.0, board.selected_cell.1)));

    stdout().execute(ResetColor)?;
    if cursor_col.2 {
        stdout().execute(SetAttribute(Attribute::Reverse))?;
    }
    execute!(
        stdout(),
        SetForegroundColor(cursor_col.0),
        SetBackgroundColor(cursor_col.1),
        cursor::MoveTo(board.selected_cell.0 * 3,     board.selected_cell.1 + 2),
        Print("["),
        cursor::MoveTo(board.selected_cell.0 * 3 + 2, board.selected_cell.1 + 2),
        Print("]"),
        ResetColor,
    )?;
    
    Ok(())
}

// Gets the color pair of a specified tile
pub fn get_tile_style(tile: Option<&Tile>) -> (Color, Color, bool) {
    // If it's none, that means we're getting the style of a bomb, so return it!!
    if tile.is_none() { return (Color::Rgb{ r: 239, g: 125, b: 87 }, Color::Reset, true) }
    match tile.unwrap() {
        Tile::Numbered(num) => match num {
            1 => (Color::Rgb { r: 112, g: 181, b: 238 }, Color::Reset, false), 
            2 => (Color::Rgb { r: 181, g: 238, b: 112 }, Color::Reset, false), 
            3 => (Color::Rgb { r: 238, g: 112, b: 181 }, Color::Reset, false), 
            4 => (Color::Rgb { r: 181, g: 112, b: 238 }, Color::Reset, false), 
            5 => (Color::Rgb { r: 238, g: 181, b: 112 }, Color::Reset, false),
            6 => (Color::Rgb { r: 255, g: 236, b: 188 }, Color::Reset, false),
            7 => (Color::Rgb { r: 234, g: 186, b: 255 }, Color::Reset, false), 
            _ => (Color::Rgb { r: 112, g: 238, b: 181 }, Color::Reset, false), 
        }
        Tile::Blank    => (Color::Rgb { r: 170, g: 170, b: 170 }, Color::Reset, false),
        Tile::Unopened => (Color::Rgb { r: 170, g: 170, b: 170 }, Color::Reset, true ),
        Tile::Flag     => (Color::Rgb { r: 224, g: 83,  b: 160 }, Color::Reset, true ),
    }
}

