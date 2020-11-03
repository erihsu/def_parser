use crate::action::action_types::Slot;
use crate::action::common_parse::{number, rect_or_polygon, tstring, ws};

use nom::bytes::complete::tag;

use nom::multi::many0;
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;

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
