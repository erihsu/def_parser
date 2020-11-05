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
