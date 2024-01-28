use std::io::Write;

use crossterm::{
    event,
    style::{Print, Stylize},
    ExecutableCommand,
};

pub mod error;
pub mod git;

fn main()
{
    let git_configurations: Vec<git::configuration::File> = git::configuration::load().unwrap();

    show_menu(
        git_configurations
            .iter()
            .map(|file| String::from(file.name.trim()))
            .collect::<Vec<String>>(),
        0,
    );
}

fn show_menu(options: Vec<String>, default: usize)
{
    let mut stdout = std::io::stdout();
    let mut current: usize = default;
    let length: usize = options.len();

    let mut cursor_position: (u16, u16) = crossterm::cursor::position().unwrap();

    //println!("{:?}", crossterm::cursor::position().unwrap());

    stdout.execute(crossterm::cursor::Hide).unwrap();
    //stdout.execute(crossterm::cursor::SavePosition).unwrap();

    loop
    {
        for (index, option) in options.iter().enumerate()
        {
            if index == current
            {
                stdout.execute(Print(" > Testing\n")).unwrap(); //.write(" > {}", String::from(option));
                                                                //println!(" > {}", String::from(option).bold().green());
                continue;
            }

            stdout.execute(Print("   Testing 2\n")).unwrap();
        }

        //println!("{:?}", crossterm::cursor::position().unwrap());

        if crossterm::event::poll(std::time::Duration::from_millis(100)).unwrap()
        {
            match crossterm::event::read().unwrap()
            {
                crossterm::event::Event::Key(event) =>
                {
                    if event.code == crossterm::event::KeyCode::Up
                    {
                        current = (current - 1) % length;
                    }

                    if event.code == crossterm::event::KeyCode::Down
                    {
                        current = (current + 1) % length;
                    }

                    if event.code == crossterm::event::KeyCode::Enter
                    {
                        break;
                    }
                }
                _ =>
                {}
            }
        }

        //println!("{:?}", crossterm::cursor::position().unwrap());

        stdout.flush().unwrap();

        /*stdout
        .flush()
        .unwrap()
        .execute(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::FromCursorDown,
        ))
        .unwrap();*/
    }
}
