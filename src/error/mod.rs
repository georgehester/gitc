pub struct Error
{
    pub message: String,
}

impl std::fmt::Debug for Error
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        return formatter.write_str(&self.message);
    }
}
