use std::path::PathBuf;

use cmd_lib::run_fun;

pub struct GitProxy {
    working_dir: PathBuf,
}

impl GitProxy {
    pub fn new(working_dir: &PathBuf) -> Self {
        Self {
            working_dir: working_dir.to_path_buf(),
        }
    }

    pub fn log(&self, after_date: &str) -> String {
        let dir = &self.working_dir;
        run_fun!(
            cd $dir;
            git log --numstat --date=short --pretty=format:"--%h--%cd--%aN--%s" --no-renames --after=$after_date
        ).unwrap()
    }
}
