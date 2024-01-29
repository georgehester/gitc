pub mod command;
pub mod error;
pub mod git;
pub mod ui;

fn main()
{
    command::switch_configuration();
    // command::list_configuration();
}
