use crate::action::action_types::Region;
use crate::action::common_parse::{number, properties, rect, tstring, ws};
use nom::branch::alt;
use nom::combinator::{map_res, opt, recognize};

use nom::bytes::complete::tag;

use nom::multi::{many0, many1};

use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

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
