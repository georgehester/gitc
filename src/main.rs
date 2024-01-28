pub mod error;
pub mod git;
pub mod ui;

fn main()
{
    let git_configurations: Vec<git::configuration::File> = git::configuration::load().unwrap();

    println!("{:?}", git_configurations);

    ui::draw_option_menu(
        git_configurations
            .iter()
            .map(|file| String::from(file.name.trim()))
            .collect::<Vec<String>>(),
        0,
    );
}
