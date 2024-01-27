use std::{ops::Deref, path::PathBuf};

pub mod error;

fn main()
{
    read_directory();
}

struct ConfigurationFile
{
    path: PathBuf,
    name: String,
}

fn read_directory()
{
    // Get the home directory of the process owner and add the gitc folder
    let mut gitc_path = homedir::get_my_home().expect("").expect("");
    gitc_path.push(".gitc");

    // Check the gitc path exists
    if !gitc_path.exists()
    {
        std::fs::create_dir_all(&gitc_path).expect("Failed to create gitc directory");
    }

    // Check the gitc path is a directory
    if !gitc_path.is_dir()
    {
        panic!("Failed as .gitc is not a directory");
    }

    // Read the gitc directory for files
    let files = std::fs::read_dir(&gitc_path).expect("");

    // Create the array to store the output
    let mut output: Vec<ConfigurationFile> = Vec::new();

    for file_result in files
    {
        let file = file_result.expect("");
        let mut file_name = file.file_name().into_string().expect("");

        if file_name.ends_with(".gitconfig")
        {
            let configuration_name = &file_name.drain(..file_name.len() - 10).collect::<String>();

            let mut file_path = std::path::PathBuf::from(&gitc_path);
            file_path.push(configuration_name);
            file_path.push(file_name);

            println!("Name: {:?}", configuration_name);
            println!("Path: {:?}", file_path);
        }
    }

    /*
    println!("Hello");

    let files: std::fs::ReadDir =
        std::fs::read_dir("~/.gitc").expect("Failed to read gitc directory");

    for file in files
    {
        println!("{:?}", file);
    }*/
}
