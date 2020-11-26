// nom
use nom::branch::permutation;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{number, tstring, ws};
use crate::def_parser::common::{comp_name, properties, source_type};
use crate::def_parser::def_types::Component;

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
            permutation((
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
                properties,
            )),
        ),
        ws(tag(";")),
    )(input)
}
