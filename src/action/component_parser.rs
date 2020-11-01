use crate::action::action_types::Component;
use crate::action::common_parse::{comp_name, float, number, qstring, source_type, tstring, ws};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::many0;

use nom::combinator::{map, opt};
use nom::multi::many1;

use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

pub fn component_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // number of Component
        Vec<Component>,
    ),
> {
    terminated(
        tuple((
            delimited(ws(tag("COMPONENTS")), number, ws(tag(";"))),
            many1(delimited(ws(tag("-")), component_member, ws(tag(";")))),
        )),
        ws(tag("END COMPONENTS")),
    )(input)
}

fn component_member(input: &str) -> IResult<&str, Component> {
    tuple((
        comp_name,
        tstring,
        opt(preceded(ws(tag("+ EEQMASTER")), tstring)),
        opt(preceded(ws(tag("+ GENERATE")), tstring)),
        opt(preceded(ws(tag("+ SOURCE")), source_type)),
        // foreign
        opt(preceded(ws(tag("+ WEIGHT")), number)),
        opt(preceded(ws(tag("+ REGION")), tstring)),
        //opt(preceded(ws(tag("+ MASKSHIFT")), tstring)),
        opt(tuple((
            map(
                preceded(ws(tag("+ HALO")), opt(ws(tag("SOFT")))),
                |res: Option<&str>| match res {
                    Some(_) => true,
                    None => false,
                },
            ),
            number,
            number,
            number,
            number,
        ))),
        opt(tuple((
            preceded(ws(tag("+ ROUTEHALO")), number),
            tstring,
            tstring,
        ))),
        many0(tuple((
            preceded(ws(tag("+ PROPERTY")), tstring),
            opt(alt((qstring, tstring))),
            opt(float),
        ))),
    ))(input)
}
