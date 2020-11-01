use crate::action::action_types::Pinprop;
use crate::action::common_parse::{float, number, qstring, tstring, ws};
use nom::combinator::{map, opt};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

pub fn pinproperty_section(input: &str) -> IResult<&str, (i32, Vec<Pinprop>)> {
    terminated(
        tuple((
            delimited(ws(tag("PINPROPERTIES")), number, ws(tag(";"))),
            many0(delimited(ws(tag("-")), pinproperty_member, ws(tag(";")))),
        )),
        ws(tag("END PINPROPERTIES")),
    )(input)
}

fn pinproperty_member(input: &str) -> IResult<&str, Pinprop> {
    tuple((
        map(tstring, |res: &str| match res {
            "PIN" => None,
            s => Some(s),
        }),
        tstring,
        many0(tuple((
            preceded(ws(tag("+ PROPERTY")), tstring),
            opt(alt((qstring, tstring))),
            opt(float),
        ))),
    ))(input)
}
