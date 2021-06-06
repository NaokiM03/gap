use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Not,
};

use crate::parser::Parser;

#[derive(Debug, Default)]
pub struct ResultParams {
    pub opt_param: BTreeMap<String, String>,
    pub pos_param: BTreeMap<String, String>,
    pub flag_param: BTreeSet<String>,
}

impl ResultParams {
    fn try_insert_short_opt(&mut self, parser: &Parser, c: char, value: &str) {
        if let Some(opt_arg) = parser
            .options_list
            .values()
            .filter(|x| x.short.is_some())
            .find(|x| x.short.unwrap() == c)
        {
            self.opt_param
                .insert(opt_arg.name.to_owned(), value.to_owned());
        }
    }

    fn try_insert_long_opt(&mut self, parser: &Parser, s: &str, value: &str) {
        if let Some(opt_arg) = parser
            .options_list
            .values()
            .filter(|x| x.long.is_some())
            .find(|x| x.long.unwrap() == s)
        {
            self.opt_param
                .insert(opt_arg.name.to_owned(), value.to_owned());
        }
    }

    fn try_insert_short_flag(&mut self, parser: &Parser, c: char) {
        if let Some(flag_arg) = parser
            .flags_list
            .values()
            .filter(|x| x.short.is_some())
            .find(|x| x.short.unwrap() == c)
        {
            self.flag_param.insert(flag_arg.name.to_owned());
        }
    }

    fn try_insert_long_flag(&mut self, parser: &Parser, s: &str) {
        if let Some(flag_arg) = parser
            .flags_list
            .values()
            .filter(|x| x.long.is_some())
            .find(|x| x.long.unwrap() == s)
        {
            self.flag_param.insert(flag_arg.name.to_owned());
        }
    }
}

impl ResultParams {
    pub fn new() -> Self {
        ResultParams {
            opt_param: BTreeMap::new(),
            pos_param: BTreeMap::new(),
            flag_param: BTreeSet::new(),
        }
    }

    pub fn try_insert_opt(&mut self, parser: &Parser, s: &str, value: &str) {
        let s = s.trim_start_matches('-');
        match s.len() {
            1 => {
                self.try_insert_short_opt(parser, s.chars().next().unwrap(), value);
            }
            _ => {
                self.try_insert_long_opt(parser, s, value);
            }
        }
    }

    pub fn try_insert_pos(&mut self, parser: &Parser, i: u8, s: &str) {
        if let Some(pos_arg) = parser.positions_list.values().find(|x| x.index == i) {
            self.pos_param.insert(pos_arg.name.to_owned(), s.to_owned());
        }
    }

    pub fn try_insert_flag(&mut self, parser: &Parser, s: &str) {
        let is_long_key = s.starts_with("--");
        let s = s.trim_start_matches('-');
        match s.len() {
            1 => {
                self.try_insert_short_flag(parser, s.chars().next().unwrap());
            }
            _ if is_long_key.not() => {
                for c in s.chars() {
                    self.try_insert_short_flag(parser, c);
                }
            }
            _ if is_long_key => {
                self.try_insert_long_flag(parser, s);
            }
            _ => unreachable!(),
        }
    }
}

impl ResultParams {
    pub fn value_of(&self, key: &str) -> Option<&str> {
        if let Some(ref value) = self.opt_param.get(key) {
            return Some(&value.as_str());
        } else if let Some(value) = self.pos_param.get(key) {
            return Some(&value.as_str());
        }
        None
    }

    pub fn is_present(&self, key: &str) -> bool {
        self.opt_param.contains_key(key)
            || self.pos_param.contains_key(key)
            || self.flag_param.contains(key)
    }
}
