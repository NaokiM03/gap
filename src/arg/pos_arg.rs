#[derive(Debug)]
pub struct PosArg<'pos> {
    pub name: &'pos str,
    pub index: u8,
}
