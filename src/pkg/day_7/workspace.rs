use std::cmp;

use super::{dir::Dir, ls::Ls};

#[derive(Clone)]
pub(crate) struct WorkSpace {
    pub dirs: Vec<Dir>,
}

impl WorkSpace {
    pub(crate) fn new() -> Self {
        let root = Dir::new(None);

        return WorkSpace { dirs: vec![root] };
    }

    pub(crate) fn add_ls(&mut self, to: usize, ls: Ls) {
        for dir in ls.dirs {
            let idx = self.dirs.clone().len();
            let dir = dir.to_string();

            self.dirs.push(Dir::new(Some(to)));
            self.dirs[to].add_dir(dir, idx);
        }

        ls.files.iter().for_each(|f| self.dirs[to].add_file(f));
    }

    pub(crate) fn get_sum_under(&self, max_size: usize) -> usize {
        let sizes = self.get_sizes();

        return sizes
            .iter()
            .filter(|s| **s < max_size)
            .fold(0, |sum, s| sum + s);
    }

    fn get_sizes(&self) -> Vec<usize> {
        return self
            .dirs
            .iter()
            .map(|dir| self.get_dir_total_size(dir))
            .collect::<Vec<_>>();
    }

    fn get_dir_total_size(&self, dir: &Dir) -> usize {
        let sub_dirs = dir.sub_directories.iter().map(|s| &self.dirs[*s.1]);

        let sub_dir_sizes =
            sub_dirs.fold(0, |sum, s| sum + self.get_dir_total_size(s));

        let file_sizes = dir.files.iter().fold(0, |sum, f| sum + f.size);
        return sub_dir_sizes + file_sizes;
    }

    pub(crate) fn get_smallest_over(
        &self,
        disk_size: usize,
        req_size: usize,
    ) -> usize {
        let sizes = self.get_sizes();
        let used_space = sizes[0];
        let available_space = disk_size - used_space;
        let req_free = req_size - available_space;

        let mut min_over = sizes[0];

        for s in sizes {
            if s > req_free {
                min_over = cmp::min(min_over, s);
            }
        }

        return min_over;
    }
}
