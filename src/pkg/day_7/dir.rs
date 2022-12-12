use std::rc::Rc;

use super::{cd::Cd, file::File, ls::Ls};

#[derive(Clone)]
pub(crate) struct Dir {
    name: String,
    parent: Option<Rc<Dir>>,
    sub_directories: Vec<Rc<Dir>>,
    files: Vec<File>,
}

impl Dir {
    pub(crate) fn new<'a>(name: String, parent: Option<Rc<Dir>>) -> Dir {
        return Dir {
            name,
            parent,
            sub_directories: vec![],
            files: vec![],
        };
    }

    pub(crate) fn get_root(&self) -> &Dir {
        match &self.parent {
            Some(parent) => parent.get_root(),
            None => self,
        }
    }

    pub(crate) fn cd(dir: Rc<Dir>, cd: Cd) -> Rc<Dir> {
        match cd.arg.as_str() {
            ".." => Dir::get_parent(dir),
            sub => dir.get_sub(sub),
        }
    }

    fn get_sub(&self, sub: &str) -> Rc<Dir> {
        return *self
            .sub_directories
            .iter()
            .find(|d| d.name == sub)
            .expect("Directory does not exist.");
    }

    fn get_parent(dir: Rc<Dir>) -> Rc<Dir> {
        match dir.parent {
            Some(parent) => parent,
            None => dir,
        }
    }

    pub(crate) fn add_ls(dir: Rc<Dir>, ls: Ls) {
        ls.dirs.iter().for_each(|d| Dir::mkdir(dir, d.to_string()));
        ls.files.iter().for_each(|file| dir.add_file(file.clone()));
    }

    fn mkdir(dir: Rc<Dir>, name: String) {
        let sub = Dir::new(name, Some(dir));
        dir.sub_directories.push(Rc::new(sub));
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
}

impl From<Vec<&str>> for Dir {
    fn from(command: Vec<&str>) -> Self {
        todo!()
    }
}
