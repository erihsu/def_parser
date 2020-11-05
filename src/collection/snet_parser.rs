// nom
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::pair;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{float, number, tstring, ws};
use crate::def_parser::common::{net_property, routing_point};
use crate::def_parser::def_types::{SNet, SpecialWireBasic, WireOption, WireShape};

pub fn snet_section(input: &str) -> IResult<&str, (i32, Vec<SNet>)> {
    delimited(
        tag("SPECIALNETS"),
        tuple((
            terminated(number, ws(tag(";"))), // numSNet
            many0(snet_member),
        )),
        tag("END SPECIALNETS"),
    )(input)
}

fn snet_member(input: &str) -> IResult<&str, SNet> {
    delimited(
        tag("-"),
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
            opt(preceded(ws(tag("+ VOLTAGE")), float)),
            opt(special_wiring),
            net_property,
        )),
        ws(tag(";")),
    )(input)
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
