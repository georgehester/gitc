const GITC_PATH: &str = ".gitc";

pub struct File
{
    pub name: String,
    pub path: std::path::PathBuf,
}

pub fn load() -> Result<Vec<File>, crate::error::Error>
{
    // Get the home directory of the process owener
    let mut gitc_path: std::path::PathBuf = match homedir::get_my_home()
    {
        Ok(path) => match path
        {
            Some(path) => path,
            None =>
            {
                return Err(crate::error::Error {
                    message: String::from("Failed to get users home directory"),
                })
            }
        },
        Err(_) =>
        {
            return Err(crate::error::Error {
                message: String::from("Failed to get users home directory"),
            })
        }
    };

    // Add the gitc path to the full path
    gitc_path.push(GITC_PATH);

    // Check the gitc path exists in the home directory
    if !gitc_path.exists()
    {
        match std::fs::create_dir_all(&gitc_path)
        {
            Ok(_) => (),
            Err(_) =>
            {
                return Err(crate::error::Error {
                    message: String::from("Failed to create gitc directory"),
                })
            }
        }
    }

    // Check the gitc path points to a directory
    if !gitc_path.is_dir()
    {
        return Err(crate::error::Error {
            message: String::from("Failed to open gitc not a directory"),
        });
    }

    // Get the list of files from the gitc path
    let files: std::fs::ReadDir = match std::fs::read_dir(&gitc_path)
    {
        Ok(files) => files,
        Err(_) =>
        {
            return Err(crate::error::Error {
                message: String::from("Failed to get files from gitc directory"),
            })
        }
    };

    let mut output: Vec<File> = Vec::new();

    for file_result in files
    {
        // Unwrap the file entry
        let file: std::fs::DirEntry = match file_result
        {
            Ok(file) => file,
            Err(_) =>
            {
                return Err(crate::error::Error {
                    message: String::from("Failed to get file from gitc directory"),
                })
            }
        };

        // Get the file name as a string
        let mut file_name: String = match file.file_name().into_string()
        {
            Ok(file_name) => file_name,
            Err(_) =>
            {
                return Err(crate::error::Error {
                    message: String::from("Failed to parse file name from gitc directory"),
                })
            }
        };

        // Check the file is a configuration file
        if file_name.ends_with(".gitconfig") && file.path().is_file()
        {
            // Get the name without extension
            let configuration_name = &file_name.drain(..file_name.len() - 10).collect::<String>();

            // Get the full path to the configuration file
            let mut file_path = std::path::PathBuf::from(&gitc_path);
            file_path.push(configuration_name);
            file_path.push(file_name);

            // Push each configuration to the output vector
            output.push(File {
                name: String::from(configuration_name),
                path: file_path,
            });
        }
    }

    return Ok(output);
}
