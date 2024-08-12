use std::collections::HashSet;
use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

pub struct FileTracker {
    pub base_dir: PathBuf,
    files: HashSet<PathBuf>,
}

impl FileTracker {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> FileTracker {
        FileTracker {
            base_dir: base_dir.as_ref().into(),
            files: HashSet::new(),
        }
    }

    pub fn create<P: AsRef<Path>, C: AsRef<str>>(
        &mut self,
        path: P,
        content: C,
    ) -> Result<(), Error> {
        let path = self.base_dir.join(path);
        if !self.files.insert(path.clone()) {
            log::error!("File written to multiple times: {}", path.display());
        }
        log::info!("Creating: {}", path.display());

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, content.as_ref())?;
        Ok(())
    }
}
