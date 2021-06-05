use core::panic;
use std::collections::{HashSet, BTreeMap};

use crate::arg::{Arg, opt_arg::OptArg, pos_arg::PosArg, flag_arg::FlagArg};

pub struct Parser<'p> {
    arg_names_list: HashSet<&'p str>,
    short_names_list: HashSet<char>,
    long_names_list: HashSet<&'p str>,
    options_list: BTreeMap<&'p str, OptArg<'p>>,
    positions_list: BTreeMap<u8, PosArg<'p>>,
    flags_list: BTreeMap<&'p str, FlagArg<'p>>,
}

impl<'p> Parser<'p> {
    fn validate_unique_arg_name(&self, s: &str) {
        if self.arg_names_list.contains(s) {
            panic!("arg name must be unique. \"{}\" is already used.", s);
        }
    }

    fn validate_unique_short_name(&self, c: char) {
        if self.short_names_list.contains(&c) {
            panic!("short name must be unique. \"{}\" is already used.", c);
        }
    }

    fn validate_unique_long_name(&self, s: &str) {
        if self.long_names_list.contains(s) {
            panic!("long name must be unique. \"{}\" is already used.", s);
        }
    }

    fn validate_unique_index(&self, i: u8) {
        if self.positions_list.contains_key(&i) {
            panic!("index must be unique. \"{}\" is already used.", i);
        }
    }

    fn validate_unique_flag(&self, s: &str) {
        if self.flags_list.contains_key(s) {
            panic!("flag must be unique. \"{}\" is already used.", s);
        }
    }
}

impl<'p> Parser<'p> {
    pub fn new() -> Self {
        Parser {
            arg_names_list: HashSet::new(),
            short_names_list: HashSet::new(),
            long_names_list: HashSet::new(),
            options_list: BTreeMap::new(),
            positions_list: BTreeMap::new(),
            flags_list: BTreeMap::new(),
        }
    }

    pub fn arg(mut self, a: Arg<'p>) -> Self {
        Self::validate_unique_arg_name(&self, a.name);
        self.arg_names_list.insert(a.name);

        if let Some(c) = a.short {
            Self::validate_unique_short_name(&self, c);
            self.short_names_list.insert(c);
        }
        
        if let Some(s) = a.long {
            Self::validate_unique_long_name(&self, s);
            self.long_names_list.insert(s);
        }

        if a.need_value {
            self.options_list.insert(a.name, OptArg {
                name: a.name,
                short: a.short,
                long: a.long,
            });
        }
        
        if let Some(i) = a.index {
            Self::validate_unique_index(&self, i);
            self.positions_list.insert(i, PosArg {
                name: a.name,
                index: i,
            });
        } 
        
        if a.flag {
            Self::validate_unique_flag(&self, a.name);
            self.flags_list.insert(a.name, FlagArg {
                name: a.name,
                short: a.short,
                long: a.long,
            });
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let actual = Parser::new().arg(Arg::new("foo").short("a")).arg_names_list.contains("foo");
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn unique_arg_name() {
        let _ = Parser::new().arg(Arg::new("foo")).arg(Arg::new("foo"));
    }

    #[test]
    #[should_panic]
    fn unique_short_name() {
        let _ = Parser::new().arg(Arg::new("foo").short("a")).arg(Arg::new("bar").short("a"));
    }

    #[test]
    #[should_panic]
    fn unique_long_name() {
        let _ = Parser::new().arg(Arg::new("foo").long("abc")).arg(Arg::new("bar").long("abc"));
    }

    #[test]
    #[should_panic]
    fn unique_index() {
        let _ = Parser::new().arg(Arg::new("foo").index(1)).arg(Arg::new("bar").index(1));
    }

    #[test]
    #[should_panic]
    fn unique_flag() {
        let _ = Parser::new().arg(Arg::new("foo").short("a").flag(true)).arg(Arg::new("bar").short("a").flag(true));
    }
}
