pub mod opt_arg;
pub mod pos_arg;
pub mod flag_arg;

pub struct Arg<'arg> {
    pub name: &'arg str,
    pub short: Option<char>,
    pub long: Option<&'arg str>,
    pub need_value: bool,
    pub index: Option<u8>,
    pub flag: bool,
}

impl<'arg> Arg<'arg> {
    fn validate_arg_name(s: &str) {
        if s.is_empty() {
            panic!("arg name is required.")
        }
        if !s.chars().all(|x| x.is_ascii_lowercase() || x.is_ascii_digit()) {
            panic!("ascii character or ascii digit can be used for arg name.")
        }
    }

    fn validate_short(s: &str) {
        if s.len() != 1 {
            panic!("single character can be used for short arg param.");
        }
        if !s.chars().all(|x| x.is_ascii_lowercase()) {
            panic!("ascii character can be used for short arg param.");
        }
    }

    fn validate_long(s: &str) {
        if s.len() <= 1 {
            panic!("more than two character can be used for long arg param.")
        }
        if !s.chars().all(|x| x.is_ascii_lowercase()) {
            panic!("ascii character can be used for long arg param.");
        }
    }

    fn validate_need_value(&self) {
        if self.is_short_and_long_name_empty() {
            panic!("expect short arg param or long arg param before define this.")
        }
    }

    fn validate_index(&self) {
        if self.is_short_or_long_name_present() {
            panic!("index cannot be used simultaneously with short arg param or long arg param.")
        }
    }

    fn validate_flag(&self) {
        if self.is_short_and_long_name_empty() {
            panic!("flag arg must need short name or long name.");
        }
    }
}

impl<'arg> Arg<'arg> {
    pub fn new(n: &'arg str) -> Self {
        Self::validate_arg_name(n);
        Arg {
            name: n,
            short: None,
            long: None,
            need_value: false,
            index: None,
            flag: false,
        }
    }

    pub fn short(mut self, s: &str) -> Self {
        Self::validate_short(s);
        self.short = s.chars().next();
        self
    }

    pub fn long(mut self, s: &'arg str) -> Self {
        Self::validate_long(s);
        self.long = Some(s);
        self
    }

    pub fn need_value(mut self) -> Self {
        Self::validate_need_value(&self);
        self.need_value = true;
        self
    }

    pub fn index(mut self, i: u8) -> Self {
        Self::validate_index(&self);
        self.index = Some(i);
        self
    }

    pub fn flag(mut self) -> Self {
        Self::validate_flag(&self);
        self.flag = true;
        self
    }

    pub fn is_short_or_long_name_present(&self) -> bool {
        self.short.is_some() || self.long.is_some()
    }

    pub fn is_short_and_long_name_empty(&self) -> bool {
        self.short.is_none() && self.long.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod name {
        use super::*;

        #[test]
        fn base() {
            let actual = Arg::new("foo").name;
            let expected = "foo";
            assert_eq!(actual, expected);
        }

        #[test]
        #[should_panic]
        fn with_empty_str() {
            let _ = Arg::new("");
        }

        #[test]
        #[should_panic]
        fn with_not_ascii() {
            let _ = Arg::new("!");
        }
    }

    mod short {
        use super::*;

        #[test]
        fn base() {
            let actual = Arg::new("foo").short("a").short;
            let expected = Some('a');
            assert_eq!(actual, expected);
        }
    
        #[test]
        #[should_panic]
        fn with_empty_str() {
            let _ = Arg::new("foo").short("");
        }
    
        #[test]
        #[should_panic]
        fn wigh_two_str() {
            let _ = Arg::new("foo").short("ab").short;
        }
    
        #[test]
        #[should_panic]
        fn with_not_ascii() {
            let _ = Arg::new("foo").short("!").short;
        }    
    }

    mod long {
        use super::*;

        #[test]
        fn base() {
            let actual = Arg::new("foo").long("abc").long;
            let expected = Some("abc");
            assert_eq!(actual, expected);
        }
    
        #[test]
        #[should_panic]
        fn with_empty_str() {
            let _ = Arg::new("foo").long("");
        }
    
        #[test]
        #[should_panic]
        fn wigh_single_char() {
            let _ = Arg::new("foo").long("a");
        }
    
        #[test]
        #[should_panic]
        fn with_not_ascii() {
            let _ = Arg::new("foo").long("a!");
        }    
    }

    mod need_value {
        use super::*;

        #[test]
        fn base() {
            let actual = Arg::new("foo").short("a").need_value().need_value;
            let expected = true;
            assert_eq!(actual, expected);
        }

        #[test]
        #[should_panic]
        fn with_no_name() {
            let _ = Arg::new("foo").need_value();
        }
    }

    mod index {
        use super::*;

        #[test]
        fn base() {
            let actual = Arg::new("foo").index(1).index;
            let expected = Some(1);
            assert_eq!(actual, expected);
        }

        #[test]
        #[should_panic]
        fn with_name() {
            let _ = Arg::new("foo").short("a").index(1);
        }
    }

    mod flag {
        use super::*;

        #[test]
        fn base() {
            let actual = Arg::new("foo").short("a").flag().flag;
            let expected = true;
            assert_eq!(actual, expected);
        }

        #[test]
        #[should_panic]
        fn with_no_name() {
            let _ = Arg::new("foo").flag();
        }
    }
}
