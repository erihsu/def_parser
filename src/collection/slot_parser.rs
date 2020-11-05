// nom
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{number, tstring, ws};
use crate::def_parser::common::rect_or_polygon;
use crate::def_parser::def_types::Slot;

pub fn slot_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // number of slot
        Vec<Slot>,
    ),
> {
    delimited(
        tag("SLOTS"),
        tuple((terminated(number, ws(tag(";"))), many0(slot_member))),
        tag("END SLOTS"),
    )(input)
}

fn slot_member(input: &str) -> IResult<&str, Slot> {
    delimited(
        tag("LAYER"),
        tuple((tstring, many0(rect_or_polygon))),
        ws(tag(";")),
    )(input)
}
