use crate::action::action_types::{RoutingPoint, SNet};
use crate::action::common_parse::{float, number, orient, qstring, tstring, ws};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::pair;

use nom::combinator::{map, opt};
use nom::multi::{many0, many1};

use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

#[derive(Debug)]
enum WireOption<'a> {
    Cover(bool),
    Fixed(bool),
    Routed(bool),
    Shield(&'a str),
}

#[derive(Debug)]
enum WireShape {
    Ring,
    PadRing,
    BlockRing,
    Stripe,
    FollowPin,
    IOWire,
    CoreWire,
    BlockWire,
    BlockageWire,
    FillWire,
    FillWireOpc,
    DrcFill,
}

type SpecialWireBasic<'a> = (
    Option<WireOption<'a>>,
    (&'a str, i32),
    Option<WireShape>,
    Option<i32>,
    Vec<RoutingPoint<'a>>,
);

pub fn snet_section(input: &str) -> IResult<&str, (i32, Vec<SNet>)> {
    terminated(
        tuple((
            delimited(ws(tag("SPECIALNETS")), number, ws(tag(";"))),
            many1(preceded(ws(tag("-")), snet_member)),
        )),
        ws(tag("END SPECIALNETS")),
    )(input)
}

fn snet_member(input: &str) -> IResult<&str, SNet> {
    tuple((
        tstring,
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
        preceded(ws(tag("+ VOLTAGE")), float),
        opt(special_wiring),
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

fn special_wire_option(input: &str) -> IResult<&str, WireOption> {
    alt((
        map(ws(tag("+ COVER")), |res: &str| WireOption::Cover(true)),
        map(ws(tag("+ FIXED")), |res: &str| WireOption::Fixed(true)),
        map(ws(tag("+ Routed")), |res: &str| WireOption::Routed(true)),
        map(pair(ws(tag("+ Shield")), tstring), |res: (&str, &str)| {
            WireOption::Shield(res.1)
        }),
    ))(input)
}

fn special_wire_shape(input: &str) -> IResult<&str, WireShape> {
    preceded(
        ws(tag("+ SHAPE")),
        alt((
            map(tag("RING"), |res: &str| WireShape::Ring),
            map(tag("PADRING"), |res: &str| WireShape::PadRing),
            map(tag("BLOCKRING"), |res: &str| WireShape::BlockRing),
            map(tag("STRIPE"), |res: &str| WireShape::Stripe),
            map(tag("FOLLOWPIN"), |res: &str| WireShape::FollowPin),
            map(tag("IOWIRE"), |res: &str| WireShape::IOWire),
            map(tag("COREWIRE"), |res: &str| WireShape::CoreWire),
            map(tag("BlOCKWIRE"), |res: &str| WireShape::BlockWire),
            map(tag("BLOCKAGEWIRE"), |res: &str| WireShape::BlockageWire),
            map(tag("FILLWIRE"), |res: &str| WireShape::FillWire),
            map(tag("FILLWIREOPC"), |res: &str| WireShape::FillWireOpc),
            map(tag("DRCFILL"), |res: &str| WireShape::DrcFill),
        )),
    )(input)
}

fn special_wire_basic(input: &str) -> IResult<&str, SpecialWireBasic> {
    tuple((
        opt(special_wire_option),
        tuple((tstring, number)),
        opt(special_wire_shape),
        opt(preceded(ws(tag("+ STYLE")), number)),
        routing_point,
    ))(input)
}

fn special_wiring(input: &str) -> IResult<&str, (SpecialWireBasic, Vec<SpecialWireBasic>)> {
    tuple((
        special_wire_basic,
        many0(preceded(ws(tag("NEW")), special_wire_basic)),
    ))(input)
}
