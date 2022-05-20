use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::convert::AsRef;
use std::ffi::OsString;
use std::fs::{DirEntry, FileType, Metadata, Permissions};
use std::iter::{IntoIterator};
use std::os::unix::fs::MetadataExt;
use once_cell::unsync::OnceCell;
use ignore::gitignore::Gitignore;

pub struct WakerEntry {
    // inner DirEntry
    inner: fs::DirEntry,
    // depth (from `1`) at which this entry was created relative to the root.
    pub depth: usize,
    // has next sibling
    pub has_next_sibling: bool,
    // inner metadata
    inner_metadata: OnceCell<Metadata>,
}

// 参考：https://docs.rs/walkdir/latest/walkdir/

impl WakerEntry {
    fn new(inner: fs::DirEntry, depth: usize, has_next_sibling: bool) -> Self {
        WakerEntry {
            inner,
            depth,
            has_next_sibling,
            inner_metadata: OnceCell::new(),
        }
    }

    fn metadata(&self) -> io::Result<&Metadata> {
        self.inner_metadata.get_or_try_init(|| { self.inner.metadata() })
    }

    pub fn path(&self) -> PathBuf {
        self.inner.path()
    }

    pub fn size(&self) -> io::Result<u64> {
        self.metadata().map(|m|m.size())
    }

    pub fn file_name(&self) -> OsString {
        self.inner.file_name()
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        self.metadata().map(|m| m.file_type())
    }

    pub fn permissions(&self) -> io::Result<Permissions> {
        self.metadata().map(|m| m.permissions())
    }
}

pub struct Walker {
    root: PathBuf,
    // 最大深度
    max_depth: Option<usize>,
    // 描述遇到 symbolic link 是否继续继续 walk
    follow_symbolic: bool,
    // 是否忽略隐藏文件和 .gitignore 里的文件
    hide_ignore: bool,
    // 按名字排序（开启后会影响效率）
    sort_by_name: bool,
}

impl Walker {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Walker {
            root: root.as_ref().to_path_buf(),
            max_depth: None,
            follow_symbolic: false,
            hide_ignore: true,
            sort_by_name: true,
        }
    }

    pub fn max_depth(&mut self, depth: Option<usize>) -> &mut Self {
        self.max_depth = depth;
        self
    }

    pub fn hide_ignore(&mut self, hidden: bool) -> &mut Self {
        self.hide_ignore = hidden;
        self
    }

    pub fn follow_symbolic(&mut self, follow: bool) -> &mut Self {
        self.follow_symbolic = follow;
        self
    }

    pub fn start(&self, cb: &dyn Fn(WakerEntry)) -> io::Result<()> {
        let gitignore =
            if self.hide_ignore {
                let mut gitignore_path: Option<PathBuf> = None;
                let mut path: Option<&Path> = Some(&self.root.as_path());
                while let Some(p) = path {
                    if p.is_dir() {
                        let tmp_path = p.join(".gitignore");
                        if tmp_path.exists() {
                            gitignore_path = Some(tmp_path.clone());
                            break
                        }
                    }
                    path = p.parent();
                }
                gitignore_path
                    .map(|p| Gitignore::new(p) )
                    .and_then(|t| {
                        if t.0.is_empty() { None } else { Some(t.0) }
                    })
            } else {
                None
            };
        self.visit_dir(self.root.as_path(),&gitignore, 1, cb)
    }

    fn visit_dir(&self, dir: &Path, gitignore: &Option<Gitignore>, depth: usize, cb: &dyn Fn(WakerEntry)) -> io::Result<()> {
        if let Some(max_depth) = self.max_depth {
            if depth > max_depth {
                return Ok(())
            }
        }

        let handle_entry = |entry: DirEntry, has_next_sibling: bool| -> io::Result<()> {
            let path = entry.path();
            if self.hide_ignore {
                if entry.file_name().to_str().unwrap_or("unknown").starts_with('.') {
                    return Ok(())
                }
                if let Some(gi) = gitignore {
                    if gi.matched(&path, path.is_dir()).is_ignore() {
                        return  Ok(())
                    }
                }
            }
            let entry = WakerEntry::new(entry, depth, has_next_sibling);
            let file_type = entry.file_type()?;
            cb(entry);
            if file_type.is_dir() {
                self.visit_dir(&path, gitignore, depth + 1, cb)?;
            } else if file_type.is_symlink() && self.follow_symbolic {
                self.visit_dir(&path, gitignore,depth + 1, cb)?;
            } else {
                // do nothing
            }
            return Ok(())
        };

        if self.sort_by_name {
            let mut entries = fs::read_dir(dir)?
                .collect::<Result<Vec<_>, io::Error>>()?;
            entries.sort_by(|a,b| a.path().cmp(&b.path()));
            let mut iter = entries.into_iter().peekable();
            while let Some(entry) = iter.next() {
                handle_entry(entry, iter.peek().is_some())?;
            }
        } else {
            let mut iter = fs::read_dir(dir)?.peekable();
            while let Some(entry) = iter.next() {
                let entry = entry?;
                handle_entry(entry, iter.peek().is_some())?;
            }
        }
        Ok(())
    }
}