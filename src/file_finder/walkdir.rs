use super::FileFinderTrait;
use std::path::Path;
use std::fmt::Debug;
use walkdir::WalkDir;
use dir_entry::WalkdirDirEntry;

#[derive(Clone)]
pub struct FileFinder {}

impl FileFinder {
    pub fn new() -> Self {
        FileFinder {}
    }
}

impl FileFinderTrait for FileFinder {
    fn walk_dir<P: AsRef<Path> + Debug + Clone, F>(&self, root: P, filter: F) -> Vec<Self::DirEntry>
        where F: Fn(&Self::DirEntry) -> bool {
        info!("Search files in directory {:?}", root);
        debug!("Start searching files in root {:?}", root);

        let result = WalkDir::new(root.clone())
            .into_iter()
            .filter_map(|entry| {
                match entry {
                    Ok(entry) => Some(WalkdirDirEntry::from_dir_entry(entry)),
                    Err(_) => None,
                }
            })
            .filter(filter)
            .collect();
        debug!("End searching files in root {:?}", root);

        result
    }
    type DirEntry = WalkdirDirEntry;
}
