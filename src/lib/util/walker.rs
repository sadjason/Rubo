
// 参考：https://docs.rs/walkdir/latest/walkdir/

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::convert::AsRef;

pub struct WakerEntry {
    // inner DirEntry
    pub inner: fs::DirEntry,
    // depth (from `1`) at which this entry was created relative to the root.
    pub depth: usize,
    // has next sibling
    pub has_next_sibling: bool
}

pub struct Walker {
    root: PathBuf,
    max_depth: usize,
    ignore_hidden: bool,
}

impl Walker {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Walker { root: root.as_ref().to_path_buf(),  max_depth: 3, ignore_hidden: true }
    }

    pub fn max_depth(&mut self, depth: usize) -> &mut Walker {
        self.max_depth = depth;
        self
    }

    pub fn start(&self, cb: &dyn Fn(WakerEntry)) -> io::Result<()> {
        self.visit_dir(self.root.as_path(),1, cb)
    }

    fn visit_dir(&self, dir: &Path, depth: usize, cb: &dyn Fn(WakerEntry)) -> io::Result<()> {
        if depth > self.max_depth {
            return Ok(())
        }
        let mut iter = fs::read_dir(dir)?.peekable();
        while let Some(entry) = iter.next() {
            let entry = entry?;
            let path = entry.path();
            if self.ignore_hidden && entry.file_name().to_str().unwrap_or(".").starts_with('.') {
                continue
            }
            cb(WakerEntry { inner: entry, depth, has_next_sibling: iter.peek().is_some() });
            if path.is_dir() {
                self.visit_dir(&path, depth + 1, cb)?;
            }
        }
        Ok(())
    }
}