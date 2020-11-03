use crate::action::action_types::{Fill, Geometry};
use crate::action::common_parse::{number, pt_list, rect_or_polygon, tstring, ws};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;

use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

pub fn fill_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // number of LAYERS
        Vec<Fill>,
    ),
> {
    delimited(
        tag("FILLS"),
        tuple((terminated(number, ws(tag(";"))), many0(fill_member))),
        ws(tag("END FILLS")),
    )(input)
}

fn fill_member(input: &str) -> IResult<&str, Fill> {
    delimited(
        tag("-"),
        alt((
            map(
                tuple((
                    preceded(tag("LAYER"), tstring),
                    opt(preceded(tag("+ MASK"), number)),
                    map(opt(ws(tag("+ OPC"))), |res: Option<&str>| match res {
                        Some(_) => true,
                        None => false,
                    }),
                    many0(rect_or_polygon),
                )),
                |res: (&str, Option<i32>, bool, Vec<Geometry>)| Fill::Layer(res),
            ),
            map(
                tuple((
                    preceded(tag("VIA"), tstring),
                    opt(preceded(tag("+ MASK"), number)),
                    map(opt(ws(tag("+ OPC"))), |res: Option<&str>| match res {
                        Some(_) => true,
                        None => false,
                    }),
                    pt_list,
                )),
                |res: (&str, Option<i32>, bool, Vec<(i32, i32)>)| Fill::Via(res),
            ),
        )),
        ws(tag(";")),
    )(input)
}
