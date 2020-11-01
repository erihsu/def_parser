use crate::action::action_types::{RoutingPoint, SNet, SpecialWiring};
use crate::action::common_parse::{float, number, orient, pt_new, qstring, rect, tstring, ws};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::pair;

use nom::combinator::{map, opt};
use nom::multi::{many0, many1};

use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

pub fn snet_section(input: &str) -> IResult<&str, (i32, Vec<SNet>)> {
    terminated(
        tuple((
            delimited(ws(tag("SPECIALNETS")), number, ws(tag(";"))),
            many1(preceded(ws(tag("-")), snet_member)),
        )),
        ws(tag("END SPECIALNETS")),
    )(input)
}

fn snet_member(input: &str) -> IResult<&str> {
    tuple(())(input)
}

fn routing_point(input: &str) -> IResult<&str, Vec<RoutingPoint>> {
    many0(alt((
        map(
            delimited(
                ws(tag("(")),
                tuple((number, number, opt(number))),
                ws(tag(")")),
            ),
            |res: (i32, i32, Option<i32>)| RoutingPoint::Pt(res),
        ),
        map(pair(tstring, opt(orient)), |res: (&str, Option<i32>)| {
            RoutingPoint::Via(res)
        }),
    )))(input)
}

fn special_wiring(input: &str) -> IResult<&str> {
    tuple((
        opt(),
        tuple((tstring, number)),
        opt(snet_shape),
        opt(preceded(ws(tag("STYLE"))), number),
        routing_point,
        opt(),
    ))(input)
}
