// nom
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{number, tstring, ws};
use crate::def_parser::common::{pt_list, rect};
use crate::def_parser::def_types::{Geometry, Via, ViaBody};

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
        pair(
            tstring,
            alt((
                map(
                    tuple((
                        preceded(ws(tag("+ VIARULE")), tstring),
                        preceded(ws(tag("+ CUTSIZE")), tuple((number, number))),
                        preceded(ws(tag("+ LAYERS")), tuple((tstring, tstring, tstring))),
                        preceded(ws(tag("+ CUTSPACING")), tuple((number, number))),
                        preceded(
                            ws(tag("+ ENCLOSURE")),
                            tuple((number, number, number, number)),
                        ),
                        opt(preceded(ws(tag("+ ROWCOL")), tuple((number, number)))),
                        opt(preceded(ws(tag("+ ORIGIN")), tuple((number, number)))),
                        opt(preceded(
                            ws(tag("+ OFFSET")),
                            tuple((number, number, number, number)),
                        )),
                        opt(preceded(ws(tag("+ PATTERN")), tstring)),
                    )),
                    |n| ViaBody::Generated(n),
                ),
                map(
                    many0(tuple((
                        tstring,
                        alt((
                            map(preceded(ws(tag("+ RECT")), rect), |n| Geometry::Rect(n)),
                            map(preceded(ws(tag("+ POLYGON")), pt_list), |n| {
                                Geometry::Polygon(n)
                            }),
                        )),
                    ))),
                    |n| ViaBody::Fixed(n),
                ),
            )),
        ),
        ws(tag(";")),
    )(input)
}
