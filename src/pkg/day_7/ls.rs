use super::file::File;

pub(crate) struct Ls {
    pub(crate) files: Vec<File>,
    pub(crate) dirs: Vec<String>,
}

impl From<Vec<String>> for Ls {
    fn from(command: Vec<String>) -> Self {
        todo!()
    }
}
