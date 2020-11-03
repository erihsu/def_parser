use crate::action::action_types::Pinprop;
use crate::action::common_parse::{number, properties, tstring, ws};
use nom::combinator::map;

use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;

pub fn pinproperty_section(input: &str) -> IResult<&str, (i32, Vec<Pinprop>)> {
    delimited(
        tag("PINPROPERTIES"),
        tuple((terminated(number, ws(tag(";"))), many0(pinproperty_member))),
        tag("END PINPROPERTIES"),
    )(input)
}

fn pinproperty_member(input: &str) -> IResult<&str, Pinprop> {
    delimited(
        tag("-"),
        tuple((
            map(tstring, |res: &str| match res {
                "PIN" => None,
                s => Some(s),
            }),
            tstring,
            properties,
        )),
        ws(tag(";")),
    )(input)
}
