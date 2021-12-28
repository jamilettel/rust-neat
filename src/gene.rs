use std::fmt;

use crate::Link;

#[derive(Clone)]
pub struct Gene<'a> {
    pub id: u32,
    pub link: &'a Link<'a>,
    pub enabled: bool,
}

impl<'a> fmt::Display for Gene<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            " Gene {} ({}) : {} ",
            self.id, if self.enabled {"enabled"} else {"disabled"},
            self.link)
    }
}
