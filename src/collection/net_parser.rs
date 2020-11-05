// nom
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{number, tstring, ws};
use crate::def_parser::common::{net_property, orient, pt_new, rect, routing_point};
use crate::def_parser::def_types::{Net, RegularWireBasic};

pub fn nets_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // numNet
        Vec<Net>,
    ),
> {
    delimited(
        tag("NETS"),
        tuple((
            terminated(number, ws(tag(";"))), // numNet
            many0(net_member),
        )),
        tag("END NETS"),
    )(input)
}

fn net_member(input: &str) -> IResult<&str, Net> {
    delimited(
        tag("-"),
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
            opt(preceded(ws(tag("+ FREQUENCY")), number)),
            net_property,
        )),
        ws(tag(";")),
    )(input)
}

fn regular_wire_basic(input: &str) -> IResult<&str, RegularWireBasic> {
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
            map(ws(tag("TAPER")), |res: _| None),
            map(preceded(ws(tag("TAPERRULE")), tstring), |res: &str| {
                Some(res)
            }),
        )),
        opt(preceded(ws(tag("STYLE")), number)),
        routing_point,
    ))(input)
}

fn regular_wiring(input: &str) -> IResult<&str, (RegularWireBasic, Vec<RegularWireBasic>)> {
    tuple((regular_wire_basic, many0(regular_wire_basic)))(input)
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
        &str,                                              // subnet name
        Vec<(&str, &str)>,                                 // pinname or vpin name
        Option<&str>,                                      // nondefaultrule
        Option<(RegularWireBasic, Vec<RegularWireBasic>)>, // regular wiring
    ),
> {
    tuple((
        preceded(ws(tag("+ SUBNET")), tstring),
        many0(tuple((tstring, tstring))),
        opt(preceded(ws(tag("+ NONDEFAULTRULE")), tstring)),
        opt(regular_wiring),
    ))(input)
}
