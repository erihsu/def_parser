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
use crate::def_parser::common::{net_property, route_body};
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
        map(ws(tag("+ COVER")), |_| WireOption::Cover(true)),
        map(ws(tag("+ FIXED")), |_| WireOption::Fixed(true)),
        map(ws(tag("+ Routed")), |_| WireOption::Routed(true)),
        map(pair(ws(tag("+ Shield")), tstring), |res: (&str, &str)| {
            WireOption::Shield(res.1)
        }),
    ))(input)
}

fn special_wire_shape(input: &str) -> IResult<&str, WireShape> {
    preceded(
        ws(tag("+ SHAPE")),
        alt((
            map(tag("RING"), |_| WireShape::Ring),
            map(tag("PADRING"), |_| WireShape::PadRing),
            map(tag("BLOCKRING"), |_| WireShape::BlockRing),
            map(tag("STRIPE"), |_| WireShape::Stripe),
            map(tag("FOLLOWPIN"), |_| WireShape::FollowPin),
            map(tag("IOWIRE"), |_| WireShape::IOWire),
            map(tag("COREWIRE"), |_| WireShape::CoreWire),
            map(tag("BlOCKWIRE"), |_| WireShape::BlockWire),
            map(tag("BLOCKAGEWIRE"), |_| WireShape::BlockageWire),
            map(tag("FILLWIRE"), |_| WireShape::FillWire),
            map(tag("FILLWIREOPC"), |_| WireShape::FillWireOpc),
            map(tag("DRCFILL"), |_| WireShape::DrcFill),
        )),
    )(input)
}

fn special_wire_basic(input: &str) -> IResult<&str, SpecialWireBasic> {
    tuple((
        opt(special_wire_option),
        tuple((tstring, number)),
        opt(special_wire_shape),
        opt(preceded(ws(tag("+ STYLE")), number)),
        route_body,
    ))(input)
}

fn special_wiring(input: &str) -> IResult<&str, (SpecialWireBasic, Vec<SpecialWireBasic>)> {
    tuple((
        special_wire_basic,
        many0(preceded(ws(tag("NEW")), special_wire_basic)),
    ))(input)
}
