use crossterm::ExecutableCommand;

pub mod error;
pub mod git;

fn main()
{
    let git_configurations: Vec<git::configuration::File> = git::configuration::load().unwrap();

    println!("{:?}", git_configurations);

    show_menu(
        git_configurations
            .iter()
            .map(|file| String::from(file.name.trim()))
            .collect::<Vec<String>>(),
        0,
    );

    println!();
}

fn show_menu(options: Vec<String>, default: usize)
{
    let mut stdout = std::io::stdout();
    let mut current: usize = default;
    let length: usize = options.len();

    crossterm::terminal::enable_raw_mode().unwrap();

    //let window_size: WindowSize = crossterm::terminal::window_size().unwrap();

    //stdout.execute(crossterm::cursor::MoveToColumn(0)).unwrap();

    //let cursor_position: (u16, u16) = crossterm::cursor::position().unwrap();

    //println!("{:?}", crossterm::cursor::position().unwrap());

    let cursor_position = crossterm::cursor::position().unwrap();
    let window_size = crossterm::terminal::window_size().unwrap();
    let extra_lines: i32 = (cursor_position.1 as i32) - (window_size.rows as i32) + (length as i32);

    if extra_lines > 0
    {
        stdout
            .execute(crossterm::terminal::ScrollUp(extra_lines as u16))
            .unwrap()
            .execute(crossterm::cursor::MoveTo(
                0,
                cursor_position.1 - (extra_lines as u16),
            ))
            .unwrap();
    }

    stdout
        .execute(crossterm::cursor::Show)
        .unwrap()
        .execute(crossterm::cursor::SavePosition)
        .unwrap();

    loop
    {
        let reset_position = stdout.execute(crossterm::cursor::SavePosition).unwrap();

        for (index, option) in options.iter().enumerate()
        {
            if index == current
            {
                stdout
                    .execute(crossterm::cursor::MoveTo(
                        0,
                        crossterm::cursor::position().unwrap().1,
                    ))
                    .unwrap()
                    .execute(crossterm::style::Print(format!(
                        " > {} {}",
                        option,
                        crossterm::cursor::position().unwrap().1,
                        //crossterm::cursor::position().unwrap().1 + (index as u16),
                        //window_size.rows,
                        //index
                    )))
                    .unwrap();
            }
            else
            {
                stdout
                    .execute(crossterm::cursor::MoveTo(
                        0,
                        crossterm::cursor::position().unwrap().1,
                    ))
                    .unwrap()
                    .execute(crossterm::style::Print(format!(
                        "   {} {}",
                        option,
                        crossterm::cursor::position().unwrap().1,
                        //crossterm::cursor::position().unwrap().1 + (index as u16),
                        //window_size.rows,
                        //index
                    )))
                    .unwrap();
            }

            if index < length
            {
                stdout
                    .execute(crossterm::cursor::MoveToRow(
                        crossterm::cursor::position().unwrap().1 + 1,
                    ))
                    .unwrap();
            }
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
                        if current <= 0
                        {
                            current = length;
                        }
                        current = current - 1;
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

        stdout
            .execute(crossterm::cursor::RestorePosition)
            .unwrap()
            .execute(crossterm::terminal::Clear(
                crossterm::terminal::ClearType::FromCursorDown,
            ))
            .unwrap();
    }

    stdout.execute(crossterm::cursor::Show).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();
}
