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

    //stdout.execute(crossterm::cursor::MoveToColumn(0)).unwrap();

    //let cursor_position: (u16, u16) = crossterm::cursor::position().unwrap();

    stdout
        .execute(crossterm::cursor::Hide)
        .unwrap()
        .execute(crossterm::cursor::SavePosition)
        .unwrap();

    loop
    {
        for (index, option) in options.iter().enumerate()
        {
            if index == current
            {
                stdout
                    .execute(crossterm::cursor::MoveToColumn(0))
                    .unwrap()
                    .execute(crossterm::style::Print(format!(" > {}\n", option)))
                    .unwrap();
                //.execute(crossterm::cursor::MoveToRow(
                //    cursor_position.1 + 1 + index as u16,
                //))
                //.unwrap();
                continue;
            }

            stdout
                .execute(crossterm::cursor::MoveToColumn(0))
                .unwrap()
                .execute(crossterm::style::Print(format!("   {}\n", option)))
                .unwrap();
            //.execute(crossterm::cursor::MoveToRow(
            //    cursor_position.1 + 1 + index as u16,
            //))
            //.unwrap();
        }

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
