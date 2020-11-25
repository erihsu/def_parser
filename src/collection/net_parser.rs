// nom
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{number, tstring, ws};
use crate::def_parser::common::{net_property, orient, pt_new, rect, route_body};
use crate::def_parser::def_types::{Net, RegularWireBasic, RegularWireStmt};

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
            // many0(vpin),
            // many0(subnet),
            opt(preceded(ws(tag("+ XTALK")), number)),
            opt(preceded(ws(tag("+ NONDEFAULTRULE")), tstring)),
            // regular_wiring,
            net_property,
        )),
        ws(tag(";")),
    )(input)
}

fn regular_wire_basic(input: &str) -> IResult<&str, RegularWireBasic> {
    tuple((
        tstring,
        alt((
            map(ws(tag("TAPER")), |_| None),
            map(preceded(ws(tag("TAPERRULE")), tstring), |res: &str| {
                Some(res)
            }),
        )),
        opt(preceded(ws(tag("STYLE")), number)),
        route_body,
    ))(input)
}

fn regular_wiring(input: &str) -> IResult<&str, RegularWireStmt> {
    many0(tuple((
        map(preceded(ws(tag("+")), tstring), |res| match res {
            "COVER" => Some(0),
            "FIXED" => Some(1),
            "ROUTED" => Some(2),
            "NOSHIELD" => Some(3),
            _ => None,
        }),
        many0(alt((
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

// fn subnet(
//     input: &str,
// ) -> IResult<
//     &str,
//     (
//         // SUBNET
//         &str,                                              // subnet name
//         Vec<(&str, &str)>,                                 // pinname or vpin name
//         Option<&str>,                                      // nondefaultrule
//         Option<(RegularWireBasic, Vec<RegularWireBasic>)>, // regular wiring
//     ),
// > {
//     tuple((
//         preceded(ws(tag("+ SUBNET")), tstring),
//         many0(tuple((tstring, tstring))),
//         opt(preceded(ws(tag("+ NONDEFAULTRULE")), tstring)),
//         opt(regular_wiring),
//     ))(input)
// }

#[cfg(test)]
mod tests {
    use crate::def_parser::def_types::*;
    use crate::def_parser::net_parser::*;
    use std::io::Read;

    #[test]
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
                ("N1", false),
                vec![(Some("I1"), "A", false), (None, "P0", false)],
                vec!["SN1", "VDD"],
                // vec![],
                Some(2),
                Some("RULE1"),
                // vec![
                // (
                //     Some(1),
                //     vec![
                //         (
                //             "M2",
                //             None,
                //             None,
                //             vec![
                //                 RouteElem::Pt((Some(14000), Some(341440), None)),
                //                 RouteElem::Pt((Some(9600), None, None)),
                //                 RouteElem::Via(((None, Some(282400), None), "nd1VIA12"))
                //             ]
                //         ),
                //         (
                //             "M1",
                //             None,
                //             None,
                //             vec![
                //                 RouteElem::Pt((Some(2400), None, None)),
                //                 RouteElem::Pt((Some(240), None, None)),
                //             ]
                //         )
                //     ]
                // ),
                // ],
                (
                    Some(1),
                    true,
                    Some(100),
                    Some("N2"),
                    Some(6),
                    Some(1),
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
}
