use dir_entry::DirEntryTrait;

pub struct Rating<'a> {
    entry: &'a DirEntryTrait,
    rating: i32,
}

impl<'a> Rating<'a> {
    pub fn new(entry: &'a DirEntryTrait, rating: i32) -> Self {
        Rating {
            entry,
            rating,
        }
    }

    pub fn entry(&self) -> &DirEntryTrait {
        self.entry
    }

    pub fn rating(&self) -> i32 {
        self.rating
    }
}
