// nom

use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{number, tstring, ws};
use crate::def_parser::common::{comp_name, properties, pt_new};
use crate::def_parser::def_types::Component;
use crate::def_parser::encoder::{
    component_location_attribute_encode, orient_encode, source_type_encode,
};

pub fn component_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // number of Component
        Vec<Component>,
    ),
> {
    delimited(
        tag("COMPONENTS"),
        tuple((
            terminated(number, ws(tag(";"))), // numComponent
            many0(component_member),
        )),
        ws(tag("END COMPONENTS")),
    )(input)
}

fn component_member(input: &str) -> IResult<&str, Component> {
    delimited(
        tag("-"),
        pair(
            tuple((comp_name, tstring)),
            tuple((
                opt(preceded(ws(tag("+ EEQMASTER")), tstring)),
                opt(source_type_encode),
                tuple((
                    component_location_attribute_encode,
                    opt(tuple((pt_new, orient_encode))),
                )),
                opt(preceded(ws(tag("+ WEIGHT")), number)),
                opt(preceded(ws(tag("+ REGION")), tstring)),
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
                properties,
            )),
        ),
        ws(tag(";")),
    )(input)
}
