use crate::action::action_types::Slot;
use crate::action::common_parse::{number, pt_list_new, tstring, ws};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
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
    terminated(
        tuple((
            delimited(ws(tag("slot_section")), number, ws(tag(";"))),
            many0(delimited(ws(tag("-")), slot_member, ws(tag(";")))),
        )),
        ws(tag("END SLOTS")),
    )(input)
}

fn slot_member(input: &str) -> IResult<&str, Slot> {
    tuple((
        preceded(ws(tag("LAYER")), tstring),
        preceded(alt((ws(tag("RECT")), ws(tag("POLYGON")))), pt_list_new),
    ))(input)
}
