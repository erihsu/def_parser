use crate::action::action_types::Ndr;
use crate::action::common_parse::{float, number, qstring, tstring, ws};
use nom::combinator::{map, opt};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

pub fn scanchain_section(input: &str) -> IResult<&str, ()> {
    terminated(
        tuple((
            delimited(ws(tag("SCANCHAINS")), number, ws(tag(";"))),
            many0(delimited(ws(tag("-")), scanchain_member, ws(tag(";")))),
        )),
        ws(tag("END SCANCHAINS")),
    )(input)
}

fn scanchain_member(input: &str) -> IResult<&str> {
    tuple((tstring,))
}
