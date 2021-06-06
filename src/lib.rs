mod arg;
mod parser;
mod result_params;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub use crate::{arg::Arg, parser::Parser};
