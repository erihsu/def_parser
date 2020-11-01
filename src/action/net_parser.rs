use crate::action::action_types::{Net, RegularWiring, RoutingPoint};
use crate::action::common_parse::{float, number, orient, pt_new, qstring, rect, tstring, ws};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::pair;

use nom::combinator::{map, opt};
use nom::multi::{many0, many1};

use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

pub fn nets_section(input: &str) -> IResult<&str, (i32, Vec<Net>)> {
    terminated(
        tuple((
            delimited(ws(tag("NETS")), number, ws(tag(";"))),
            many1(delimited(ws(tag("-")), net_member, ws(tag(";")))),
        )),
        ws(tag("END GROUPS")),
    )(input)
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

fn regular_wiring(input: &str) -> IResult<&str, (RegularWiring, Vec<RegularWiring>)> {
    tuple((
        tuple((
            map(tstring, |res: &str| match res {
                "COVER" => Some(0),
                "FIXED" => Some(1),
                "ROUTED" => Some(2),
                "NOSHIELD" => Some(3),
                _ => None,
            }),
            tstring,
            alt((
                map(ws(tag("TAPER")), |res: &str| None),
                map(preceded(ws(tag("TAPERRULE")), tstring), |res: &str| {
                    Some(res)
                }),
            )),
            opt(preceded(ws(tag("STYLE")), number)),
            routing_point,
        )),
        many0(tuple((
            map(tstring, |res: &str| match res {
                "COVER" => Some(0),
                "FIXED" => Some(1),
                "ROUTED" => Some(2),
                "NOSHIELD" => Some(3),
                _ => None,
            }),
            tstring,
            alt((
                map(ws(tag("TAPER")), |res: &str| None),
                map(preceded(ws(tag("TAPERRULE")), tstring), |res: &str| {
                    Some(res)
                }),
            )),
            opt(preceded(ws(tag("STYLE")), number)),
            routing_point,
        ))),
    ))(input)
}

fn vpin(
    input: &str,
) -> IResult<
    &str,
    (
        // VPIN
        &str,                     // vpin name
        Option<&str>,             // layer name
        ((i32, i32), (i32, i32)), // vpin geometry
        Option<i32>,              // 0: placed ; 1: fixed ; 2: covered
        Option<(i32, i32)>,       // vpin location
        Option<i32>,              // orient
    ),
> {
    tuple((
        preceded(ws(tag("+ VPIN")), tstring),
        opt(preceded(ws(tag("+ LAYER")), tstring)),
        rect,
        map(tstring, |res: &str| match res {
            "PLACED" => Some(0),
            "FIXED" => Some(1),
            "COVERED" => Some(2),
            _ => None,
        }),
        opt(pt_new),
        opt(orient),
    ))(input)
}

fn subnet(
    input: &str,
) -> IResult<
    &str,
    (
        // SUBNET
        &str,                                        // subnet name
        Vec<(&str, &str)>,                           // pinname or vpin name
        Option<&str>,                                // nondefaultrule
        Option<(RegularWiring, Vec<RegularWiring>)>, // regular wiring
    ),
> {
    tuple((
        preceded(ws(tag("+ SUBNET")), tstring),
        many0(tuple((tstring, tstring))),
        opt(preceded(ws(tag("+ NONDEFAULTRULE")), tstring)),
        opt(regular_wiring),
    ))(input)
}

fn net_member(input: &str) -> IResult<&str, Net> {
    tuple((
        map(tstring, |res: &str| match res {
            "MUSTJOIN" => (res, true),
            _ => (res, false),
        }),
        many0(delimited(
            ws(tag("(")),
            tuple((
                tstring,
                tstring,
                map(
                    opt(ws(tag("+ SYNTHESIZED"))),
                    |res: Option<&str>| match res {
                        Some(_) => true,
                        None => false,
                    },
                ),
            )),
            ws(tag(")")),
        )),
        many0(preceded(ws(tag("+ SHIELDNET")), tstring)),
        many0(vpin),
        many0(subnet),
        opt(preceded(ws(tag("+ XTALK")), number)),
        map(
            preceded(ws(tag("+ SOURCE")), tstring),
            |res: &str| match res {
                "DIST" => Some(0),
                "NETLIST" => Some(1),
                "TEST" => Some(2),
                "TIMING" => Some(3),
                "USER" => Some(4),
                _ => None,
            },
        ),
        map(opt(ws(tag("+ FIXEDBUMP"))), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        opt(preceded(ws(tag("+ FREQUENCY")), number)),
        opt(preceded(ws(tag("+ ORIGINAL")), tstring)),
        map(preceded(ws(tag("+ USE")), tstring), |res: &str| match res {
            "ANALOG" => Some(0),
            "CLOCK" => Some(1),
            "GROUND" => Some(2),
            "POWER" => Some(3),
            "RESET" => Some(4),
            "SCAN" => Some(5),
            "SIGNAL" => Some(6),
            "TIEOFF" => Some(7),
            _ => None,
        }),
        map(
            preceded(ws(tag("+ PATTERN")), tstring),
            |res: &str| match res {
                "BALANCED" => Some(0),
                "STEINER" => Some(1),
                "TRUNK" => Some(2),
                "WIREDLOGIC" => Some(3),
                _ => None,
            },
        ),
        opt(preceded(ws(tag("+ ESTCAP")), number)),
        opt(preceded(ws(tag("+ WEIGHT")), number)),
        many0(tuple((
            preceded(ws(tag("+ PROPERTY")), tstring),
            opt(alt((qstring, tstring))),
            opt(float),
        ))),
    ))(input)
}
