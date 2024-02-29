use std::{io::{stdout, Stdout, Write}, time::Duration};

use crossterm::{
    cursor::MoveTo, event::{self, poll, read, Event, KeyCode}, execute, style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}, terminal::{disable_raw_mode, enable_raw_mode, size}, ExecutableCommand, QueueableCommand
};

struct World{
    player_c: u16,
    player_l: u16,
}

fn draw(mut sc: &Stdout ,world: &World){
    sc.queue(MoveTo(world.player_c, world.player_l));
    sc.queue(Print("p"));

    sc.flush();
}

fn main() -> std::io::Result<()> {


    // init the screen
    let mut sc = stdout();
    let (maxc , maxl) = size().unwrap();

    enable_raw_mode();
    
    sc
        .execute(SetBackgroundColor(Color::Green))?
        // .execute(Print("Styled text here."))?
        .execute(ResetColor)?;

    // init the game
    let mut world = World {
        player_c: 0, player_l: maxl / 2
    };

    

    loop {
    if poll(Duration:: from_millis(10))? {
let key = read().unwrap();

match key {
    Event::Key(event) => {
        match event.code {
            KeyCode::Enter => { 
                // todo change bird position
                // if world.player_c >= 0 {world.player_c += 4};
                // if world.player_l > maxl-4 {world.player_l +=4};
            },
            KeyCode::End => {
                break;
            }
            _ => {}
        }
    },
    _ => {}
}
    }else{

    }

    // physics()

    draw(&sc, &world);

    };

    disable_raw_mode();
    Ok(())
}