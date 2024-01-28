pub mod error;
pub mod git;
pub mod ui;

fn main()
{
    switch_configuration();
}

fn switch_configuration()
{
    // Load the current git configurations
    let git_configurations: Vec<git::configuration::File> = git::configuration::load().unwrap();

    println!("Select a git configuration");

    // Prompt user to select a configuration from the list
    let selected = ui::draw_option_menu(
        git_configurations
            .iter()
            .map(|file| String::from(file.name.trim()))
            .collect::<Vec<String>>(),
        0,
    );

    println!();
    println!(
        "Git configuration updated to {}",
        git_configurations
            .iter()
            .map(|file| String::from(file.name.trim()))
            .collect::<Vec<String>>()[selected]
    );
}
