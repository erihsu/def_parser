use crate::action::action_types::Via;
use crate::action::common_parse::{float, number, pt_list_new, qstring, tstring, ws};
use nom::combinator::{map, opt};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

pub fn via_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // viaNum
        Vec<Via>,
    ),
> {
    terminated(
        tuple((
            delimited(ws(tag("VIAS")), number, ws(tag(";"))),
            many0(delimited(ws(tag("-")), via_member, ws(tag(";")))),
        )),
        ws(tag("END VIAS")),
    )(input)
}

fn via_member(input: &str) -> IResult<&str, Via> {
    tuple((
        tstring,
        opt(preceded(ws(tag("+ VIARULE")), tstring)),
        opt(preceded(ws(tag("+ CUTSIZE")), tuple((number, number)))),
        opt(preceded(
            ws(tag("+ LAYERS")),
            tuple((tstring, tstring, tstring)),
        )),
        opt(preceded(ws(tag("+ CUTSPACING")), tuple((number, number)))),
        opt(preceded(
            ws(tag("+ ENCLOSURE")),
            tuple((number, number, number, number)),
        )),
        opt(preceded(ws(tag("+ ROWCOL")), tuple((number, number)))),
        opt(preceded(ws(tag("+ ORIGIN")), tuple((number, number)))),
        opt(preceded(ws(tag("+ OFFSET")), tuple((number, number)))),
        opt(preceded(ws(tag("+ PATTERN")), tstring)),
        many0(tuple((
            preceded(alt((ws(tag("+ RECT")), ws(tag("+ POLYGON")))), tstring),
            opt(preceded(ws(tag("+ MASK")), number)),
            pt_list_new,
        ))),
    ))(input)
}
