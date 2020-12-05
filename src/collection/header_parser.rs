use nom::combinator::opt;
use nom::sequence::tuple;
use nom::IResult;

use super::design_parser::{busbit_chars, divider_char, version_num};

pub fn header_section(input: &str) -> IResult<&str, (Option<f64>, Option<&str>, Option<&str>)> {
    tuple((opt(version_num), opt(divider_char), opt(busbit_chars)))(input)
}
