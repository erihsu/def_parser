use crate::action::common_parse::{float, number, pt, qstring, tstring, ws};

use nom::branch::alt;
use nom::bytes::complete::tag;

use nom::combinator::{map, opt};
use nom::multi::{many0, many1};

use crate::action::action_types::Group;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

pub fn group_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // numGroups
        Vec<Group>,
    ),
> {
    terminated(
        tuple((
            delimited(ws(tag("GROUPS")), number, ws(tag(";"))),
            many1(preceded(ws(tag("-")), group_member)),
        )),
        ws(tag("END GROUPS")),
    )(input)
}

fn group_member(input: &str) -> IResult<&str, Group> {
    tuple((
        tstring,
        many1(tstring),
        tuple((
            opt(preceded(ws(tag("+ SOFT MAXHALFPERIMETER")), number)),
            opt(preceded(ws(tag("MAXX")), number)),
            opt(preceded(ws(tag("MAXY")), number)),
        )),
        tuple((
            opt(preceded(ws(tag("+ REGION")), tstring)),
            opt(map(
                preceded(ws(tag("+ REGION")), tuple((pt, pt))),
                |res: ((&str, &str), (&str, &str))| {
                    let xl = (res.0).0.parse::<i32>().unwrap();
                    let yl = (res.0).1.parse::<i32>().unwrap();
                    let xh = (res.1).0.parse::<i32>().unwrap();
                    let yh = (res.1).1.parse::<i32>().unwrap();
                    (xl, yl, xh, yh)
                },
            )),
        )),
        many0(tuple((
            preceded(ws(tag("+ PROPERTY")), tstring),
            opt(alt((qstring, tstring))),
            opt(float),
        ))),
    ))(input)
}
