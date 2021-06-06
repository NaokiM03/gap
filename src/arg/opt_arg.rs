#[derive(Debug)]
pub struct OptArg<'opt> {
    pub name: &'opt str,
    pub short: Option<char>,
    pub long: Option<&'opt str>,
}
