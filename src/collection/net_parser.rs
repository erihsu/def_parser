// nom
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;

// def
use super::base::{float, itstring, number, tstring, ws};
use super::common::{properties, pt_new, rect, route_body};
use super::def_types::{Net, NetProperty, RegularWireBasic, RegularWireStmt, SubNet, Vpin};
use super::encoder::{
    net_global_attribute_encode, net_pattern_encode, orient_encode, pin_location_attribute_encode,
    source_type_encode, use_mode_encode,
};

pub fn net_section(
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
        pair(
            tuple((
                tstring,
                many0(delimited(
                    ws(tag("(")),
                    tuple((
                        map(tstring, |n| match n {
                            "PIN" => None,
                            n => Some(n),
                        }),
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
            )),
            tuple((
                many0(preceded(ws(tag("+ SHIELDNET")), tstring)),
                many0(vpin),
                many0(subnet),
                opt(preceded(ws(tag("+ XTALK")), number)),
                opt(preceded(ws(tag("+ NONDEFAULTRULE")), itstring)),
                many0(regular_wiring),
                net_property,
            )),
        ),
        ws(tag(";")),
    )(input)
}

fn regular_wire_basic(input: &str) -> IResult<&str, RegularWireBasic> {
    tuple((
        tstring,
        alt((
            map(preceded(ws(tag("TAPERRULE")), tstring), |res: &str| {
                Some(res)
            }),
            map(opt(ws(tag("TAPER"))), |_| None),
        )),
        opt(preceded(ws(tag("STYLE")), number)),
        route_body,
    ))(input)
}

fn regular_wiring(input: &str) -> IResult<&str, RegularWireStmt> {
    tuple((
        preceded(ws(tag("+")), net_global_attribute_encode),
        many1(alt((
            preceded(ws(tag("NEW")), regular_wire_basic),
            regular_wire_basic,
        ))),
    ))(input)
}

fn subnet_regular_wiring(input: &str) -> IResult<&str, RegularWireStmt> {
    tuple((
        net_global_attribute_encode,
        many1(alt((
            preceded(ws(tag("NEW")), regular_wire_basic),
            regular_wire_basic,
        ))),
    ))(input)
}

fn vpin(input: &str) -> IResult<&str, Vpin> {
    tuple((
        preceded(ws(tag("+ VPIN")), tstring),
        preceded(ws(tag("LAYER")), tstring),
        rect,
        pin_location_attribute_encode,
        pt_new,
        orient_encode,
    ))(input)
}

fn subnet(input: &str) -> IResult<&str, SubNet> {
    tuple((
        tuple((
            preceded(ws(tag("+ SUBNET")), tstring),
            many1(delimited(
                ws(tag("(")),
                map(pair(tstring, tstring), |n| match n.0 {
                    "VPIN" => (None, Some(n.1), None),
                    "PIN" => (None, None, Some(n.1)),
                    _ => (Some(n.0), None, Some(n.1)),
                }),
                ws(tag(")")),
            )),
        )),
        tuple((
            opt(preceded(ws(tag("NONDEFAULTRULE")), itstring)),
            many0(subnet_regular_wiring),
        )),
    ))(input)
}

fn net_property(input: &str) -> IResult<&str, NetProperty> {
    tuple((
        opt(source_type_encode),
        map(opt(ws(tag("+ FIXEDBUMP"))), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        opt(preceded(ws(tag("+ FREQUENCY")), float)),
        opt(preceded(ws(tag("+ ORIGINAL")), tstring)),
        opt(use_mode_encode),
        opt(net_pattern_encode),
        opt(preceded(ws(tag("+ ESTCAP")), number)),
        opt(preceded(ws(tag("+ WEIGHT")), number)),
        properties,
    ))(input)
}

// #[cfg(test)]
// mod tests {

//     use super::def_types::PropValue;
//     use super::def_types::RouteElem;
//     use super::net_parser::*;
//     use std::io::Read;

//     #[test]
//     fn test_net_section() {
//         let mut input_def = std::fs::File::open("tests/net_test.def").unwrap();
//         let mut data = String::new();
//         input_def.read_to_string(&mut data).unwrap();
//         let result = net_section(&data).unwrap();

//         let net_section = result.1;

//         let num = net_section.0;
//         let nets = net_section.1;

//         assert_eq!(num, 6);

//         let net_1_feature = (
//             vec![], // sheildnet
//             vec![], // vpin
//             vec![], // subnet
//             None,   // xtalk
//             None,   // ndr
//             vec![],
//             (
//                 Some(4),
//                 false, // fixedbump
//                 None,  // frequency
//                 None,  // original
//                 None,  // use
//                 None,  // pattern
//                 None,  // EXTCAP
//                 None,  // WEIGHT
//                 vec![],
//             ), // netproperty
//         );

//         let net_2_feature = (
//             vec!["SN1", "VDD"], // shieldnet
//             vec![
//                 (
//                     "N1_VP0",
//                     Some("M3"),
//                     ((-333, -333), (333, 333)),
//                     Some(0),
//                     Some((189560, 27300)),
//                     Some(0),
//                 ),
//                 ("N1_VP8", None, ((-333, -333), (333, 333)), None, None, None),
//             ], // vpin
//             vec![(
//                 (
//                     "N1_SUB0",
//                     vec![
//                         (Some("I2"), None, Some("A")),
//                         (None, None, Some("P1")),
//                         (None, Some("N1_VP9"), None),
//                     ],
//                 ),
//                 (
//                     Some("RULE1"),
//                     vec![(
//                         2,
//                         vec![(
//                             "M1",
//                             None,
//                             None,
//                             vec![
//                                 RouteElem::Pt((Some(168280), Some(63300), Some(700))),
//                                 RouteElem::Via(((None, Some(64500), None), "M1_M2")),
//                                 RouteElem::Via(((Some(169400), None, Some(800)), "M2_M3")),
//                             ],
//                         )],
//                     )],
//                 ),
//             )], // subnet
//             Some(2),
//             Some("RULE1"),
//             vec![(
//                 2,
//                 vec![
//                     (
//                         "M2",
//                         None,
//                         None,
//                         vec![
//                             RouteElem::Pt((Some(14000), Some(341440), None)),
//                             RouteElem::Pt((Some(9600), None, None)),
//                             RouteElem::Via(((None, Some(282400), None), "nd1VIA12")),
//                             RouteElem::Via(((Some(2400), None, None), "TURNM1_1")),
//                         ],
//                     ),
//                     (
//                         "M1",
//                         None,
//                         None,
//                         vec![
//                             RouteElem::Pt((Some(2400), Some(282400), None)),
//                             RouteElem::Pt((Some(240), None, None)),
//                         ],
//                     ),
//                 ],
//             )],
//             (
//                 Some(1),
//                 true,
//                 Some(100),
//                 Some("N2"),
//                 Some(6),
//                 Some(1),
//                 Some(1500000),
//                 Some(100),
//                 vec![
//                     ("strprop", PropValue::SValue("\"aString\"")),
//                     ("intprop", PropValue::IValue(1)),
//                     ("realprop", PropValue::RValue(1.1)),
//                     ("intrangeprop", PropValue::IValue(25)),
//                     ("realrangeprop", PropValue::RValue(25.25)),
//                 ],
//             ),
//         );

//         assert_eq!(
//             nets,
//             vec![
//                 (
//                     (
//                         "SCAN",
//                         vec![
//                             (Some("scancell1"), "PA10", true),
//                             (Some("scancell2"), "PA2", true)
//                         ]
//                     ),
//                     net_1_feature,
//                 ),
//                 (
//                     ("N1", vec![(Some("I1"), "A", false), (None, "P0", false)],),
//                     net_2_feature,
//                 )
//             ]
//         );
//     }
// }
