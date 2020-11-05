// nom
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{float, number, qstring, tstring, ws};
use crate::def_parser::common::pt_list;
use crate::def_parser::parser_types::Via;

pub fn via_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // viaNum
        Vec<Via>,
    ),
> {
    delimited(
        tag("VIAS"),
        tuple((
            terminated(number, ws(tag(";"))), // numVia
            many0(via_member),
        )),
        tag("END VIAS"),
    )(input)
}

fn via_member(input: &str) -> IResult<&str, Via> {
    delimited(
        tag("-"),
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
                pt_list,
            ))),
        )),
        ws(tag(";")),
    )(input)
}
