// nom
use nom::branch::{alt, permutation};
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::{many0, many1};

use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{float, number, tstring, ws};
use crate::def_parser::common::{properties, route_body};
use crate::def_parser::def_types::{SNet, SNetProperty, SpecialWireBasic, SpecialWireStmt};
use crate::def_parser::encoder::{
    net_pattern_encode, snet_global_attribute_encode, snet_shape_encode, source_type_encode,
    use_mode_encode,
};

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
            special_wiring,
            snet_property,
        )),
        ws(tag(";")),
    )(input)
}

fn special_wire_shape(input: &str) -> IResult<&str, i32> {
    map(preceded(ws(tag("+ SHAPE")), tstring), |res| {
        snet_shape_encode(res).unwrap()
    })(input)
}

fn special_wire_basic(input: &str) -> IResult<&str, SpecialWireBasic> {
    tuple((
        tstring,
        number,
        opt(special_wire_shape),
        opt(preceded(ws(tag("+ STYLE")), number)),
        route_body,
    ))(input)
}

fn special_wiring(input: &str) -> IResult<&str, SpecialWireStmt> {
    many0(tuple((
        map(preceded(ws(tag("+")), tstring), |res| {
            snet_global_attribute_encode(res).unwrap()
        }),
        many1(alt((
            preceded(ws(tag("NEW")), special_wire_basic),
            special_wire_basic,
        ))),
    )))(input)
}

fn snet_property(input: &str) -> IResult<&str, SNetProperty> {
    permutation((
        map(preceded(ws(tag("+ SOURCE")), tstring), |res| {
            source_type_encode(res).unwrap()
        }),
        map(opt(ws(tag("+ FIXEDBUMP"))), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        opt(preceded(ws(tag("+ ORIGINAL")), tstring)),
        map(preceded(ws(tag("+ USE")), tstring), |res| {
            use_mode_encode(res).unwrap()
        }),
        map(preceded(ws(tag("+ PATTERN")), tstring), |res| {
            net_pattern_encode(res).unwrap()
        }),
        opt(preceded(ws(tag("+ ESTCAP")), number)),
        opt(preceded(ws(tag("+ WEIGHT")), number)),
        properties,
    ))(input)
}
