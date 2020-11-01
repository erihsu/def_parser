use crate::action::action_types::Region;
use crate::action::common_parse::{float, number, qstring, rect, tstring, ws};
use nom::branch::alt;
use nom::combinator::{map, opt};

use nom::bytes::complete::tag;

use nom::multi::many0;

use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

pub fn region_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // numRegions
        Vec<Region>,
    ),
> {
    terminated(
        tuple((
            delimited(ws(tag("REGIONS")), number, ws(tag(";"))),
            many0(preceded(ws(tag("-")), region_member)),
        )),
        ws(tag("END REGIONS")),
    )(input)
}

fn region_member(input: &str) -> IResult<&str, Region> {
    tuple((
        tstring,
        many0(rect),
        map(
            terminated(ws(tag("+ TYPE")), tstring),
            |res: &str| match res {
                "FENCE" => 0,
                "GUIDE" => 1,
                _ => 2,
            },
        ),
        many0(tuple((
            preceded(ws(tag("+ PROPERTY")), tstring),
            opt(alt((qstring, tstring))),
            opt(float),
        ))),
    ))(input)
}
