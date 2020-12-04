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
                    many0(alt((
                        tuple((
                            preceded(ws(tag("+ RECT")), tstring),
                            map(rect, |u| Geometry::Rect(u)),
                        )),
                        tuple((
                            preceded(ws(tag("+ POLYGON")), tstring),
                            map(pt_list, |u| Geometry::Polygon(u)),
                        )),
                    ))),
                    |n| ViaBody::Fixed(n),
                ),
            )),
        ),
        ws(tag(";")),
    )(input)
}

#[cfg(test)]
mod tests {

    use crate::def_parser::via_parser::*;
    use std::io::Read;

    #[test]
    fn test_via_section() {
        let mut input_def = std::fs::File::open("tests/via_test.def").unwrap();
        let mut data = String::new();
        input_def.read_to_string(&mut data).unwrap();
        let result = via_section(&data).unwrap();
        let via_section = result.1;
        let num = via_section.0;
        let _vias = via_section.1;
        assert_eq!(num, 6);
    }
}
