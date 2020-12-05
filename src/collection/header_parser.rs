use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::opt;

use nom::sequence::{delimited, tuple};
use nom::IResult;

use super::base::{float, ws};

pub fn header_section(input: &str) -> IResult<&str, (Option<f64>, Option<&str>, Option<&str>)> {
    tuple((opt(version_num), opt(divider_char), opt(busbit_chars)))(input)
}

fn divider_char(input: &str) -> IResult<&str, &str> {
    delimited(
        ws(tag("DIVIDERCHAR")),
        alt((ws(tag("/")), ws(tag("\\")), ws(tag("%")), ws(tag("$")))),
        ws(tag(";")),
    )(input)
}
fn busbit_chars(input: &str) -> IResult<&str, &str> {
    delimited(
        ws(tag("BUSBITCHARS")),
        alt((ws(tag("[]")), ws(tag("{}")), ws(tag("<>")))),
        ws(tag(";")),
    )(input)
}

// parse version number
fn version_num(
    input: &str,
) -> IResult<
    &str,
    f64, // version number
> {
    delimited(ws(tag("VERSION")), float, ws(tag(";")))(input)
}
