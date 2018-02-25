use dir_entry::DirEntryTrait;

#[derive(Debug)]
pub struct Rating<'a> {
    entry: &'a DirEntryTrait,
    rating: isize,
}

impl<'a> Rating<'a> {
    pub fn new(entry: &'a DirEntryTrait, rating: isize) -> Self {
        Rating {
            entry,
            rating,
        }
    }

    pub fn entry(&self) -> &DirEntryTrait {
        self.entry
    }

    pub fn rating(&self) -> isize {
        self.rating
    }
}
