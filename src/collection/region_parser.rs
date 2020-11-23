// nom
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map_res, opt, recognize};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{number, tstring, ws};
use crate::def_parser::common::{properties, rect};
use crate::def_parser::def_types::Region;

pub fn region_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // total number of regions
        Vec<Region>,
    ),
> {
    delimited(
        tag("REGIONS"),
        tuple((
            terminated(number, ws(tag(";"))), // total number of regions
            many0(region_member),
        )),
        tag("END REGIONS"),
    )(input)
}

fn region_member(input: &str) -> IResult<&str, Region> {
    delimited(
        tag("-"),
        tuple((
            tstring,
            many1(rect),
            opt(preceded(ws(tag("+ TYPE")), region_type)),
            properties,
        )),
        ws(tag(";")),
    )(input)
}

fn region_type(input: &str) -> IResult<&str, i32> {
    map_res(
        recognize(alt((tag("FENCE"), tag("GUIDE")))),
        |res: &str| match res {
            "FENCE" => Ok(0),
            "GUIDE" => Ok(1),
            _ => Err(0),
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::def_parser::def_types::*;
    use crate::def_parser::region_parser::*;
    use std::io::Read;

    #[test]
    fn test_region_section() {
        let mut input_def = std::fs::File::open("tests/region_test.def").unwrap();
        let mut data = String::new();
        input_def.read_to_string(&mut data).unwrap();
        let result = region_section(&data).unwrap();

        let region_section = result.1;

        let num = region_section.0;
        let regions = region_section.1;

        assert_eq!(num, 2);
        assert_eq!(
            regions,
            vec![
                (
                    "region1",
                    vec![((-500, -500), (300, 100)), ((500, 500), (1000, 1000))],
                    Some(0),
                    vec![
                        ("strprop", PropValue::SValue("\"aString\"")),
                        ("intprop", PropValue::IValue(1)),
                        ("realprop", PropValue::RValue(1.1)),
                        ("intrangeprop", PropValue::IValue(25)),
                        ("realrangeprop", PropValue::RValue(25.25))
                    ]
                ),
                ("region2", vec![((4000, 0), (5000, 1000))], Some(1), vec![],),
            ]
        );
    }
}
