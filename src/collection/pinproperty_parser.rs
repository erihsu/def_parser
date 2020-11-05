// nom
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{number, tstring, ws};
use crate::def_parser::common::properties;
use crate::def_parser::def_types::Pinprop;

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
