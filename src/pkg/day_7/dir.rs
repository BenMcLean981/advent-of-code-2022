use std::collections::HashMap;

use super::{cd::Cd, file::File};

#[derive(Clone)]
pub(crate) struct Dir {
    name: String,
    parent: Option<usize>,
    pub sub_directories: HashMap<String, usize>,
    pub files: Vec<File>,
}

impl Dir {
    pub(crate) fn new<'a>(name: String, parent: Option<usize>) -> Dir {
        return Dir {
            name,
            parent,
            sub_directories: HashMap::new(),
            files: vec![],
        };
    }

    pub(crate) fn cd(&self, cd: Cd) -> usize {
        match cd.arg.as_str() {
            ".." => self.get_parent(),
            sub => self.get_sub(sub),
        }
    }

    fn get_sub(&self, sub: &str) -> usize {
        if sub == "/" {
            return 0;
        } else {
            return *self
                .sub_directories
                .get(sub)
                .expect("The given subdirectory does not exist.");
        }
    }

    fn get_parent(&self) -> usize {
        match self.parent {
            Some(parent) => parent,
            None => 0,
        }
    }

    pub(crate) fn add_file(&mut self, f: &File) {
        self.files.push(f.clone());
    }

    pub(crate) fn add_dir(&mut self, d: String, idx: usize) {
        self.sub_directories.insert(d, idx);
    }
}
