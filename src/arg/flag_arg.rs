#[derive(Debug)]
pub struct FlagArg<'flag> {
    pub name: &'flag str,
    pub short: Option<char>,
    pub long: Option<&'flag str>,
}
