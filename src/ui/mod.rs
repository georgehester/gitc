fn setup_terminal(stdout: &mut std::io::Stdout)
{
    // Hide cursor and enable raw mode
    crossterm::terminal::enable_raw_mode().unwrap();
    crossterm::execute!(stdout, crossterm::cursor::Hide).unwrap();
}

fn cleanup_terminal(stdout: &mut std::io::Stdout)
{
    // Reset cursor and disable raw mode
    crossterm::execute!(stdout, crossterm::cursor::Show).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();

    // Print terminating byte
    println!();
}

pub fn draw_option_menu(options: Vec<String>, default: usize) -> usize
{
    // Create stdout
    let mut stdout = std::io::stdout();

    // Setup terminal for output
    setup_terminal(&mut stdout);

    // Get the current data from the input
    let mut current: usize = default;
    let length: usize = options.len();

    // Calculate if we need more space to show output
    let scroll: i32 = (crossterm::cursor::position().unwrap().1 as i32)
        - (crossterm::terminal::window_size().unwrap().rows as i32)
        + (length as i32);

    // Scroll if we need more space
    if scroll > 0
    {
        crossterm::execute!(
            &mut stdout,
            crossterm::terminal::ScrollUp(scroll as u16),
            crossterm::cursor::MoveTo(
                0,
                crossterm::cursor::position().unwrap().1 - (scroll as u16),
            )
        )
        .unwrap();
    }

    // Save cursors current position
    crossterm::execute!(&mut stdout, crossterm::cursor::SavePosition).unwrap();

    // Render loop
    loop
    {
        // Output the options to the terminal
        for (index, option) in options.iter().enumerate()
        {
            // Highlight the current option
            if index == current
            {
                crossterm::execute!(
                    &mut stdout,
                    crossterm::cursor::MoveTo(0, crossterm::cursor::position().unwrap().1),
                    crossterm::style::SetAttribute(crossterm::style::Attribute::Bold),
                    crossterm::style::Print(" -> "),
                    crossterm::style::SetForegroundColor(crossterm::style::Color::Green),
                    crossterm::style::Print(format!("{}", option)),
                    crossterm::style::ResetColor,
                    crossterm::style::SetAttribute(crossterm::style::Attribute::Reset),
                )
                .unwrap();
            }
            else
            {
                crossterm::execute!(
                    &mut stdout,
                    crossterm::cursor::MoveTo(0, crossterm::cursor::position().unwrap().1),
                    crossterm::style::Print(format!("    {}", option)),
                )
                .unwrap();
            }

            // Move cursor down a line if required
            if index < length
            {
                crossterm::execute!(
                    &mut stdout,
                    crossterm::cursor::MoveToRow(crossterm::cursor::position().unwrap().1 + 1)
                )
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

                    if !event.modifiers.is_empty()
                    {
                        if event
                            .modifiers
                            .contains(crossterm::event::KeyModifiers::CONTROL)
                            && event.code == crossterm::event::KeyCode::Char('c')
                        {
                            cleanup_terminal(&mut stdout);
                            std::process::exit(0)
                        }
                    }
                }
                _ =>
                {}
            }
        }

        // Restore cursor position and clear the current output
        crossterm::execute!(
            &mut stdout,
            crossterm::cursor::RestorePosition,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::FromCursorDown,)
        )
        .unwrap();
    }

    cleanup_terminal(&mut stdout);

    // Return selected option
    return current;
}

pub fn draw_list(list: Vec<String>, selected: usize)
{
    // Create stdout
    let mut stdout = std::io::stdout();

    // Setup terminal for output
    setup_terminal(&mut stdout);

    for (index, item) in list.iter().enumerate()
    {
        if index == selected
        {
            crossterm::execute!(
                &mut stdout,
                crossterm::cursor::MoveTo(0, crossterm::cursor::position().unwrap().1),
                crossterm::style::SetAttribute(crossterm::style::Attribute::Bold),
                crossterm::style::Print(" -> "),
                crossterm::style::SetForegroundColor(crossterm::style::Color::Green),
                crossterm::style::Print(format!("{}", item)),
                crossterm::style::ResetColor,
                crossterm::style::SetAttribute(crossterm::style::Attribute::Reset),
            )
            .unwrap();
        }
        else
        {
            crossterm::execute!(
                &mut stdout,
                crossterm::cursor::MoveTo(0, crossterm::cursor::position().unwrap().1),
                crossterm::style::Print(format!("    {}", item)),
            )
            .unwrap();
        }

        if index < list.len() - 1
        {
            crossterm::execute!(&mut stdout, crossterm::style::Print("\n"),).unwrap();
        }
    }

    cleanup_terminal(&mut stdout);
}
