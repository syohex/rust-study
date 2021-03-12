mod game;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use game::Universe;
use std::io::stdout;
use std::time::Duration;

fn main() -> Result<()> {
    let mut game = Universe::new(5, 5);
    game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
    execute!(
        stdout(),
        EnterAlternateScreen,
        SetForegroundColor(Color::Magenta),
        Hide,
    )?;

    loop {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(_) => break,
                _ => {}
            }
        } else {
            execute!(
                stdout(),
                Clear(ClearType::All),
                MoveTo(0, 0),
                Print(&game),
                Print("Press enter to exit...")
            )?;
            game.tick();
        }
    }

    execute!(stdout(), ResetColor, Show, LeaveAlternateScreen)?;
    Ok(())
}
