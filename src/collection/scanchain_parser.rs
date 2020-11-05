// nom
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{float, number, qstring, tstring, ws};
use crate::def_parser::def_types::Ndr;

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
