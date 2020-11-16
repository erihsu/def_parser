// nom
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{number, tstring, ws};
use crate::def_parser::common::{properties, rect};
use crate::def_parser::def_types::{Group, GroupRegion};

pub fn group_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // numGroups
        Vec<Group>,
    ),
> {
    delimited(
        tag("GROUPS"),
        tuple((
            terminated(
                number, // numGroups
                ws(tag(";")),
            ),
            many0(group_member),
        )),
        ws(tag("END GROUPS")),
    )(input)
}

fn group_member(input: &str) -> IResult<&str, Group> {
    delimited(
        tag("-"),
        tuple((
            tstring,
            many1(tstring),
            opt(preceded(tag("+ SOFT MAXHALFPERIMETER"), number)),
            opt(preceded(tag("MAXX"), number)),
            opt(preceded(tag("MAXY"), number)),
            alt((
                map(preceded(tag("+ REGION"), tstring), |res: &str| {
                    GroupRegion::PreDefined(res)
                }),
                map(
                    preceded(tag("+ REGION"), rect),
                    |res: ((i32, i32), (i32, i32))| GroupRegion::NewDefined(res),
                ),
            )),
            properties,
        )),
        ws(tag(";")),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::def_parser::def_types::*;
    use crate::def_parser::group_parser::*;
    use std::io::Read;

    #[test]
    fn test_group_section() {
        let mut input_def = std::fs::File::open("tests/group_test.def").unwrap();
        let mut data = String::new();
        input_def.read_to_string(&mut data).unwrap();
        let result = group_section(&data).unwrap();

        let group_section = result.1;

        let num = group_section.0;
        let groups = group_section.1;

        assert_eq!(num, 3);
        assert_eq!(
            groups,
            vec![
                (
                    "group1",
                    vec!["I3", "I2"],
                    Some(4000),
                    Some(100000),
                    Some(100000),
                    GroupRegion::PreDefined("region1"),
                    vec![
                        ("strprop", PropValue::SValue("\"aString\"")),
                        ("intprop", PropValue::IValue(1)),
                        ("realprop", PropValue::RValue(1.1)),
                        ("intrangeprop", PropValue::IValue(25)),
                        ("realrangeprop", PropValue::RValue(25.25))
                    ]
                ),
                (
                    "group2",
                    vec!["I4"],
                    Some(4000),
                    None,
                    None,
                    GroupRegion::NewDefined(((0, 0), (100, 100))),
                    vec![],
                ),
                (
                    "region2",
                    vec!["I7", "I8"],
                    None,
                    None,
                    None,
                    GroupRegion::PreDefined("region2"),
                    vec![],
                ),
            ]
        );
    }
}
