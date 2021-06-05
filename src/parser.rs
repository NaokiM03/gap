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
}
