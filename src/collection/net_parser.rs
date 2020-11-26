// nom
use nom::branch::{alt, permutation};
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{number, tstring, ws};
use crate::def_parser::common::{orient, properties, pt_new, rect, route_body};
use crate::def_parser::def_types::{Net, NetProperty, RegularWireBasic, RegularWireStmt};
use crate::def_parser::encoder::{
    net_global_attribute_encode, net_pattern_encode, pin_location_attribute_encode,
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
            many0(preceded(ws(tag("+ SHIELDNET")), tstring)),
            many0(vpin),
            many0(subnet),
            opt(preceded(ws(tag("+ XTALK")), number)),
            opt(preceded(ws(tag("+ NONDEFAULTRULE")), tstring)),
            regular_wiring,
            net_property,
        )),
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
    many0(tuple((
        map(preceded(ws(tag("+")), tstring), |res| {
            net_global_attribute_encode(res).unwrap()
        }),
        many1(alt((
            preceded(ws(tag("NEW")), regular_wire_basic),
            regular_wire_basic,
        ))),
    )))(input)
}

fn subnet_regular_wiring(input: &str) -> IResult<&str, RegularWireStmt> {
    many0(tuple((
        map(tstring, |res| net_global_attribute_encode(res).unwrap()),
        many1(alt((
            preceded(ws(tag("NEW")), regular_wire_basic),
            regular_wire_basic,
        ))),
    )))(input)
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
        i32,                      // 0: placed ; 1: fixed ; 2: covered
        Option<(i32, i32)>,       // vpin location
        Option<i32>,              // orient
    ),
> {
    tuple((
        preceded(ws(tag("+ VPIN")), tstring),
        opt(preceded(ws(tag("+ LAYER")), tstring)),
        rect,
        map(tstring, |res| pin_location_attribute_encode(res).unwrap()),
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
        &str,                       // subnet name
        (Option<&str>, &str, bool), // pinname or vpin name
        Option<&str>,               // nondefaultrule
        RegularWireStmt,            // regular wiring
    ),
> {
    tuple((
        preceded(ws(tag("+ SUBNET")), tstring),
        delimited(
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
        ),
        opt(preceded(ws(tag("+ NONDEFAULTRULE")), tstring)),
        subnet_regular_wiring,
    ))(input)
}

fn net_property(input: &str) -> IResult<&str, NetProperty> {
    permutation((
        map(preceded(ws(tag("+ SOURCE")), tstring), |res| {
            source_type_encode(res).unwrap()
        }),
        map(opt(ws(tag("+ FIXEDBUMP"))), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        opt(preceded(ws(tag("+ FREQUENCY")), number)),
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

#[cfg(test)]
mod tests {
    use crate::def_parser::def_types::*;
    use crate::def_parser::net_parser::*;
    use std::io::Read;

    // #[test]
    fn test_net_section() {
        let mut input_def = std::fs::File::open("tests/net_test.def").unwrap();
        let mut data = String::new();
        input_def.read_to_string(&mut data).unwrap();
        let result = net_section(&data).unwrap();

        let net_section = result.1;

        let num = net_section.0;
        let nets = net_section.1;

        assert_eq!(num, 6);
        assert_eq!(
            nets,
            vec![(
                "N1",
                vec![(Some("I1"), "A", false), (None, "P0", false)],
                vec!["SN1", "VDD"],
                vec![],
                vec![],
                Some(2),
                Some("RULE1"),
                vec![(
                    2,
                    vec![
                        (
                            "M2",
                            None,
                            None,
                            vec![
                                RouteElem::Pt((Some(14000), Some(341440), None)),
                                RouteElem::Pt((Some(9600), None, None)),
                                RouteElem::Via(((None, Some(282400), None), "nd1VIA12")),
                                RouteElem::Via(((Some(2400), None, None), "TURNM1_1")),
                            ]
                        ),
                        (
                            "M1",
                            None,
                            None,
                            vec![
                                RouteElem::Pt((Some(2400), Some(282400), None)),
                                RouteElem::Pt((Some(240), None, None)),
                            ]
                        )
                    ]
                ),],
                (
                    1,
                    true,
                    Some(100),
                    Some("N2"),
                    6,
                    1,
                    Some(1500000),
                    Some(100),
                    vec![
                        ("strprop", PropValue::SValue("\"aString\"")),
                        ("intprop", PropValue::IValue(1)),
                        ("realprop", PropValue::RValue(1.1)),
                        ("intrangeprop", PropValue::IValue(25)),
                        ("realrangeprop", PropValue::RValue(25.25))
                    ]
                )
            )]
        );
    }

    #[test]
    fn test_regular_wiring() {
        assert_eq!(
            regular_wiring(
                "  + ROUTED
    M2 ( 14000 341440 ) ( 9600 * ) ( * 282400 ) nd1VIA12
    NEW M1 TAPER ( 2400 282400 ) ( 240 * )"
            )
            .unwrap(),
            (
                "",
                vec![(
                    2,
                    vec![
                        (
                            "M2",
                            None,
                            None,
                            vec![
                                RouteElem::Pt((Some(14000), Some(341440), None)),
                                RouteElem::Pt((Some(9600), None, None)),
                                RouteElem::Via(((None, Some(282400), None), "nd1VIA12"))
                            ]
                        ),
                        (
                            "M1",
                            None,
                            None,
                            vec![
                                RouteElem::Pt((Some(2400), Some(282400), None)),
                                RouteElem::Pt((Some(240), None, None)),
                            ]
                        )
                    ]
                ),],
            )
        );
    }

    #[test]
    fn test_net_property() {
        assert_eq!(
            net_property(
                "  + SOURCE NETLIST
  + FIXEDBUMP
  + FREQUENCY 100
  + ORIGINAL N2
  + USE SIGNAL
  + PATTERN STEINER
  + ESTCAP 1500000
  + WEIGHT 100
  + PROPERTY strprop \"aString\" 
  + PROPERTY intprop 1 
  + PROPERTY realprop 1.1 
  + PROPERTY intrangeprop 25
  + PROPERTY realrangeprop 25.25 "
            )
            .unwrap(),
            (
                "",
                (
                    1,
                    true,
                    Some(100),
                    Some("N2"),
                    6,
                    1,
                    Some(1500000),
                    Some(100),
                    vec![
                        ("strprop", PropValue::SValue("\"aString\"")),
                        ("intprop", PropValue::IValue(1)),
                        ("realprop", PropValue::RValue(1.1)),
                        ("intrangeprop", PropValue::IValue(25)),
                        ("realrangeprop", PropValue::RValue(25.25))
                    ]
                )
            )
        );
    }
}
