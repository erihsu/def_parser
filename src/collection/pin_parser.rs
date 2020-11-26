// nom
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{number, qstring, tstring, ws};
use crate::def_parser::common::{orient, pt_list, pt_new, rect};
use crate::def_parser::def_types::{Pin, Port, PortElem};
use crate::def_parser::encoder::{
    pin_antenna_model, pin_direction, pin_location_attribute_encode, use_mode_encode,
};

pub fn pin_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // number of PINS
        Vec<Pin>,
    ),
> {
    delimited(
        tag("PINS"),
        tuple((terminated(number, ws(tag(";"))), many0(pin_member))),
        ws(tag("END PINS")),
    )(input)
}

fn pin_member(input: &str) -> IResult<&str, Pin> {
    delimited(
        tag("-"),
        tuple((
            tuple((tstring, preceded(ws(tag("+ NET")), tstring))),
            map(opt(ws(tag("+ SPECIAL"))), |n| match n {
                Some(_) => true,
                None => false,
            }),
            map(preceded(ws(tag("+ DIRECTION")), tstring), |n| {
                pin_direction(n).unwrap()
            }),
            opt(preceded(ws(tag("+ NETEXPR")), qstring)),
            opt(preceded(ws(tag("+ SUPPLYSENSITIVITY")), tstring)),
            opt(preceded(ws(tag("+ GROUNDSENSITIVITY")), tstring)),
            opt(map(preceded(ws(tag("+ USE")), tstring), |n| {
                use_mode_encode(n).unwrap()
            })),
            many0(preceded(
                ws(tag("+ ANTENNAPINPARTIALMETALAREA")),
                tuple((number, opt(preceded(tag("LAYER"), tstring)))),
            )),
            many0(preceded(
                ws(tag("+ ANTENNAPINPARTIALMETALSIDEAREA")),
                tuple((number, opt(preceded(tag("LAYER"), tstring)))),
            )),
            many0(preceded(
                ws(tag("+ ANTENNAPINPARTIALCUTAREA")),
                tuple((number, opt(preceded(tag("LAYER"), tstring)))),
            )),
            many0(preceded(
                ws(tag("+ ANTENNAPINDIFFAREA")),
                tuple((number, opt(preceded(tag("LAYER"), tstring)))),
            )),
            opt(map(preceded(ws(tag("+ ANTENNAMODEL")), tstring), |n| {
                pin_antenna_model(n).unwrap()
            })),
            many0(preceded(
                ws(tag("+ ANTENNAPINGATEAREA")),
                tuple((number, opt(preceded(tag("LAYER"), tstring)))),
            )),
            many0(preceded(
                ws(tag("+ ANTENNAPINMAXAREACAR")),
                tuple((number, preceded(tag("LAYER"), tstring))),
            )),
            many0(preceded(
                ws(tag("+ ANTENNAPINMAXSIDEAREACAR")),
                tuple((number, preceded(tag("LAYER"), tstring))),
            )),
            many0(preceded(
                ws(tag("+ ANTENNAPINMAXCUTCAR")),
                tuple((number, preceded(tag("LAYER"), tstring))),
            )),
            many0(pin_port),
        )),
        ws(tag(";")),
    )(input)
}

fn pin_port(input: &str) -> IResult<&str, Port> {
    preceded(
        ws(tag("+ PORT")),
        tuple((
            many0(pin_port_element),
            map(preceded(ws(tag("+")), tstring), |n| {
                pin_location_attribute_encode(n).unwrap()
            }),
            pt_new,
            orient,
        )),
    )(input)
}

fn pin_port_element(input: &str) -> IResult<&str, PortElem> {
    alt((
        map(
            tuple((
                preceded(ws(tag("+ LAYER")), tstring),
                opt(alt((
                    preceded(tag("SPACING"), number),
                    preceded(tag("DESIGNRULEWIDTH"), number),
                ))),
                rect,
            )),
            |n| PortElem::Layer(n),
        ),
        map(
            tuple((
                preceded(ws(tag("+ POLYGON")), tstring),
                opt(alt((
                    preceded(tag("SPACING"), number),
                    preceded(tag("DESIGNRULEWIDTH"), number),
                ))),
                pt_list,
            )),
            |n| PortElem::Polygon(n),
        ),
        map(tuple((preceded(ws(tag("+ VIA")), tstring), pt_new)), |n| {
            PortElem::Via(n)
        }),
    ))(input)
}
