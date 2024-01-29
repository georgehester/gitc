pub fn switch_configuration()
{
    // Load the current git configurations
    let git_configurations: Vec<crate::git::configuration::File> =
        crate::git::configuration::load().unwrap();

    println!("Select a git configuration");

    // Prompt user to select a configuration from the list
    let selected = crate::ui::draw_option_menu(
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

pub fn list_configuration()
{
    // Load the current git configurations
    let git_configurations: Vec<crate::git::configuration::File> =
        crate::git::configuration::load().unwrap();

    crate::ui::draw_list(
        git_configurations
            .iter()
            .map(|file| String::from(file.name.trim()))
            .collect::<Vec<String>>(),
        0,
    );
}
