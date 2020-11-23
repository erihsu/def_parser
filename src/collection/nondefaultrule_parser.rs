// nom
use nom::bytes::complete::tag;
use nom::combinator::{map, opt, recognize};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{float, number, tstring, ws};
use crate::def_parser::common::properties;
use crate::def_parser::def_types::Ndr;

pub fn ndr_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // number of Ndr
        Vec<Ndr>,
    ),
> {
    delimited(
        tag("NONDEFAULTRULES"),
        tuple((terminated(number, ws(tag(";"))), many0(ndr_member))),
        tag("END NONDEFAULTRULES"),
    )(input)
}

fn ndr_member(input: &str) -> IResult<&str, Ndr> {
    delimited(
        tag("-"),
        tuple((
            tstring,
            map(
                opt(ws(tag("+ HARDSPACING"))),
                |res: Option<&str>| match res {
                    Some(_) => true,
                    None => false,
                },
            ),
            many0(ndr_layer),
            // be cautious with space ending in the following two tag parser
            many0(preceded(recognize(tag("+ VIA ")), tstring)),
            many0(preceded(recognize(tag("+ VIARULE ")), tstring)),
            many0(preceded(
                recognize(tag("+ MINCUTS")),
                tuple((tstring, number)),
            )),
            properties,
        )),
        ws(tag(";")),
    )(input)
}

fn ndr_layer(
    input: &str,
) -> IResult<
    &str,
    (
        &str, // name
        f64,  // width
        f64,  // diagwidth
        f64,  // spacing
        f64,  // wireext
    ),
> {
    tuple((
        preceded(tag("+ LAYER"), tstring),
        preceded(tag("WIDTH"), float),
        map(
            opt(preceded(ws(tag("DIAGWIDTH")), float)),
            |res: Option<f64>| {
                match res {
                    Some(n) => n,
                    None => 0.0, // default 0 accroding to defref
                }
            },
        ),
        preceded(ws(tag("SPACING")), float),
        map(
            opt(preceded(ws(tag("WIREEXT")), float)),
            |res: Option<f64>| {
                match res {
                    Some(n) => n,
                    None => 0.0, // default 0 according to defref
                }
            },
        ),
    ))(input)
}

#[cfg(test)]
mod tests {
    use crate::def_parser::def_types::*;
    use crate::def_parser::nondefaultrule_parser::*;
    use std::io::Read;

    #[test]
    fn test_ndr_section() {
        let mut input_def = std::fs::File::open("tests/ndr_test.def").unwrap();
        let mut data = String::new();
        input_def.read_to_string(&mut data).unwrap();
        let result = ndr_section(&data).unwrap();

        let ndr_section = result.1;

        let num = ndr_section.0;
        let ndrs = ndr_section.1;

        assert_eq!(num, 1);
        assert_eq!(
            ndrs,
            vec![
                (
                    "DEFAULT",
                    false,
                    vec![
                        ("METAL1", 10.1, 8.01, 2.2, 1.1),
                        ("M2", 10.1, 0.0, 2.2, 0.0),
                        ("M3", 11.1, 0.0, 3.2, 0.0)
                    ],
                    vec!["M1_M2", "M2_M3"],
                    vec!["VIAGEN12"],
                    vec![("V1", 2)],
                    vec![
                        ("strprop", PropValue::SValue("\"aString\"")),
                        ("intprop", PropValue::IValue(1)),
                        ("realprop", PropValue::RValue(1.1)),
                        ("intrangeprop", PropValue::IValue(25)),
                        ("realrangeprop", PropValue::RValue(25.25))
                    ]
                ),
                (
                    "RULE2",
                    true,
                    vec![
                        ("METAL1", 10.1, 8.01, 2.2, 1.1),
                        ("M2", 10.1, 0.0, 2.2, 0.0),
                        ("M3", 11.1, 0.0, 3.2, 0.0)
                    ],
                    vec!["M1_M2", "M2_M3"],
                    vec!["VIAGEN12"],
                    vec![("V1", 2)],
                    vec![
                        ("strprop", PropValue::SValue("\"aString\"")),
                        ("intprop", PropValue::IValue(1)),
                        ("realprop", PropValue::RValue(1.1)),
                        ("intrangeprop", PropValue::IValue(25)),
                        ("realrangeprop", PropValue::RValue(25.25))
                    ]
                ),
            ]
        );
    }
}
