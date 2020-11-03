use crate::action::action_types::Style;
use crate::action::common_parse::{number, pt_list, ws};

use nom::bytes::complete::tag;

use nom::multi::many0;

use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;

pub fn styles_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // total number of styles
        Vec<Style>,
    ),
> {
    delimited(
        tag("STYLES"),
        tuple((
            terminated(number, ws(tag(";"))), // total number of styles
            many0(style_member),
        )),
        tag("END STYLES"),
    )(input)
}

fn style_member(input: &str) -> IResult<&str, Style> {
    delimited(tag("- STYLE"), tuple((number, pt_list)), ws(tag(";")))(input)
}
