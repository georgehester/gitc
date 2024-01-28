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
}

fn show_menu(options: Vec<String>, default: usize)
{
    // Get the current data from the input
    let mut current: usize = default;
    let length: usize = options.len();

    // Get stdout
    let mut stdout = std::io::stdout();

    // Set terminal to raw mode
    crossterm::terminal::enable_raw_mode().unwrap();

    // Calculate if we need more space to show output
    let scroll: i32 = (crossterm::cursor::position().unwrap().1 as i32)
        - (crossterm::terminal::window_size().unwrap().rows as i32)
        + (length as i32);

    // Scroll if we need more space
    if scroll > 0
    {
        stdout
            .execute(crossterm::terminal::ScrollUp(scroll as u16))
            .unwrap()
            .execute(crossterm::cursor::MoveTo(
                0,
                crossterm::cursor::position().unwrap().1 - (scroll as u16),
            ))
            .unwrap();
    }

    // Hide the cursor and save its current position
    stdout
        .execute(crossterm::cursor::Hide)
        .unwrap()
        .execute(crossterm::cursor::SavePosition)
        .unwrap();

    // Render loop
    loop
    {
        // Output the options to the terminal
        for (index, option) in options.iter().enumerate()
        {
            // Highlight the current option
            if index == current
            {
                stdout
                    .execute(crossterm::cursor::MoveTo(
                        0,
                        crossterm::cursor::position().unwrap().1,
                    ))
                    .unwrap()
                    .execute(crossterm::style::Print(format!(" > {}", option)))
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
                    .execute(crossterm::style::Print(format!("   {}", option)))
                    .unwrap();
            }

            // Move cursor down a line if required
            if index < length
            {
                stdout
                    .execute(crossterm::cursor::MoveToRow(
                        crossterm::cursor::position().unwrap().1 + 1,
                    ))
                    .unwrap();
            }
        }

        // Poll for user input
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

        // Restore cursor position and clear the current output
        stdout
            .execute(crossterm::cursor::RestorePosition)
            .unwrap()
            .execute(crossterm::terminal::Clear(
                crossterm::terminal::ClearType::FromCursorDown,
            ))
            .unwrap();
    }

    // Reset cursor view and disable raw mode
    stdout.execute(crossterm::cursor::Show).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();

    // Print terminating byte
    println!();
}
