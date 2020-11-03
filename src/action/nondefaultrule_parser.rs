use crate::action::action_types::Ndr;
use crate::action::common_parse::{number, properties, tstring, ws};
use nom::combinator::{map, opt};

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
    delimited(
        tag("NONDEFAULTRULES"),
        tuple((terminated(number, ws(tag(";"))), many0(ndr_member))),
        tag("END COMPONENTS"),
    )(input)
}

fn ndr_member(input: &str) -> IResult<&str, Ndr> {
    delimited(
        tag("-"),
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
        i32,  // width
        i32,  // diagwidth
        i32,  // spacing
        i32,  // wireext
    ),
> {
    tuple((
        preceded(tag("+ LAYER"), tstring),
        preceded(tag("WIDTH"), number),
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
