use crate::core_version;
use std::str::FromStr;

pub struct Parser;

impl Parser {}

impl From<core_version::Parser> for Parser {
    fn from(partial: core_version::Parser) -> Self {
        todo!()
    }
}

impl Parser {
    pub fn parse() {
        let parser = core_version::Parser;

        if
        /* has more */
        true {
            todo!("parse last component & expect done")
        } else {
            todo!("error")
        }
    }
}
