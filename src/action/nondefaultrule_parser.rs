use crate::action::action_types::Ndr;
use crate::action::common_parse::{float, number, qstring, tstring, ws};
use nom::combinator::{map, opt};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

pub fn ndr_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // number of Ndr
        Vec<Ndr>,
    ),
> {
    terminated(
        tuple((
            delimited(ws(tag("NONDEFAULTRULES")), number, ws(tag(";"))),
            many0(delimited(ws(tag("-")), ndr_member, ws(tag(";")))),
        )),
        ws(tag("END COMPONENTS")),
    )(input)
}

fn ndr_member(input: &str) -> IResult<&str, Ndr> {
    tuple((
        tstring,
        map(opt(ws(tag("HARDSPACING"))), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        many0(ndr_layer),
        many0(preceded(ws(tag("+ VIA")), tstring)),
        preceded(ws(tag("+ VIARULE")), tstring),
        preceded(ws(tag("+ MINCUTS")), tuple((tstring, number))),
        many0(tuple((
            preceded(ws(tag("+ PROPERTY")), tstring),
            opt(alt((qstring, tstring))),
            opt(float),
        ))),
    ))(input)
}

fn ndr_layer(input: &str) -> IResult<&str, (&str, i32, i32, i32, i32)> {
    tuple((
        preceded(ws(tag("+ LAYER")), tstring),
        preceded(ws(tag("+ WIDTH")), number),
        map(
            opt(preceded(ws(tag("+ DIAGWIDTH")), number)),
            |res: Option<i32>| {
                match res {
                    Some(n) => n,
                    None => 0, // default 0 accroding to defref
                }
            },
        ),
        preceded(ws(tag("+ SPACING")), number),
        map(
            opt(preceded(ws(tag("+ WIREEXT")), number)),
            |res: Option<i32>| {
                match res {
                    Some(n) => n,
                    None => 0, // default 0 according to defref
                }
            },
        ),
    ))(input)
}
