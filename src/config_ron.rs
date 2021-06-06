use serde::Deserialize;

use crate::{result_params::ResultParams, Arg, Parser};

#[derive(Debug, Deserialize)]
pub struct RonArg {
    pub name: String,
    pub short: Option<String>,
    pub long: Option<String>,
    pub need_value: Option<bool>,
    pub flag: Option<bool>,
    pub index: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub args: Vec<RonArg>,
}

impl Config {
    fn from_ron(ron: &str) -> Option<Self> {
        let ron = ron::from_str::<Config>(ron);
        match ron {
            Ok(c) => Some(c),
            Err(_) => None,
        }
    }

    pub fn ron_to_result(ron: String, args: String) -> Option<ResultParams> {
        let conf = Config::from_ron(&ron);

        if let Some(conf) = conf {
            let mut parser = Parser::new();
            for x in conf.args.iter() {
                let mut arg = Arg::new(&x.name);
                if let Some(s) = &x.short {
                    arg = arg.short(&s);
                }
                if let Some(s) = &x.long {
                    arg = arg.long(&s);
                }
                if x.need_value.is_some() {
                    arg = arg.need_value();
                }
                if x.flag.is_some() {
                    arg = arg.flag();
                }
                if let Some(i) = &x.index {
                    arg = arg.index(*i);
                }

                parser = parser.arg(arg);
            }

            Some(parser.input_command(&args).execute())
        } else {
            None
        }
    }
}
