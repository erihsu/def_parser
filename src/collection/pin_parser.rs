// nom
use nom::branch::{alt, permutation};
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{number, qstring, tstring, ws};
use crate::def_parser::common::{pt_list, pt_new, rect};
use crate::def_parser::def_types::{Pin, PinAntenna, Port, PortElem};
use crate::def_parser::encoder::{
    orient_encode, pin_antenna_model_encode, pin_direction_encode, pin_location_attribute_encode,
    use_mode_encode,
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
            tuple((
                // map(opt(ws(tag("+ SPECIAL"))), |n| match n {
                //     Some(_) => true,
                //     None => false,
                // }),
                // opt(preceded(ws(tag("+")), pin_direction_encode)),
                // opt(preceded(ws(tag("+ NETEXPR")), qstring)),
                // opt(preceded(ws(tag("+ SUPPLYSENSITIVITY")), tstring)),
                // opt(preceded(ws(tag("+ GROUNDSENSITIVITY")), tstring)),
                // opt(preceded(ws(tag("+")), use_mode_encode)),
                // many0(pin_port),
                many0(pin_antenna),
            )),
        )),
        ws(tag(";")),
    )(input)
}

fn pin_port(input: &str) -> IResult<&str, Port> {
    preceded(
        ws(tag("+ PORT")),
        tuple((
            many0(pin_port_element),
            preceded(ws(tag("+")), pin_location_attribute_encode),
            pt_new,
            orient_encode,
        )),
    )(input)
}

fn pin_antenna(input: &str) -> IResult<&str, PinAntenna> {
    alt((
        map(
            preceded(
                ws(tag("+ ANTENNAPINPARTIALMETALAREA")),
                tuple((number, opt(preceded(ws(tag("LAYER")), tstring)))),
            ),
            |n| PinAntenna::PartialMetalArea(n),
        ),
        map(
            preceded(
                ws(tag("+ ANTENNAPINPARTIALMETALSIDEAREA")),
                tuple((number, opt(preceded(ws(tag("LAYER")), tstring)))),
            ),
            |n| PinAntenna::PartialMetalSideArea(n),
        ),
        map(
            preceded(
                ws(tag("+ ANTENNAPINPARTIALCUTAREA")),
                tuple((number, opt(preceded(ws(tag("LAYER")), tstring)))),
            ),
            |n| PinAntenna::PartialCutArea(n),
        ),
        map(
            preceded(
                ws(tag("+ ANTENNAPINDIFFAREA")),
                tuple((number, opt(preceded(ws(tag("LAYER")), tstring)))),
            ),
            |n| PinAntenna::DiffArea(n),
        ),
        map(
            opt(preceded(
                ws(tag("+ ANTENNAMODEL")),
                pin_antenna_model_encode,
            )),
            |n| PinAntenna::Model(n),
        ),
        map(
            preceded(
                ws(tag("+ ANTENNAPINGATEAREA")),
                tuple((number, opt(preceded(ws(tag("LAYER")), tstring)))),
            ),
            |n| PinAntenna::GateArea(n),
        ),
        map(
            preceded(
                ws(tag("+ ANTENNAPINMAXAREACAR")),
                tuple((number, preceded(ws(tag("LAYER")), tstring))),
            ),
            |n| PinAntenna::MaxAreaCar(n),
        ),
        map(
            preceded(
                ws(tag("+ ANTENNAPINMAXSIDEAREACAR")),
                tuple((number, preceded(ws(tag("LAYER")), tstring))),
            ),
            |n| PinAntenna::MaxSideAreaCar(n),
        ),
        map(
            preceded(
                ws(tag("+ ANTENNAPINMAXCUTCAR")),
                tuple((number, preceded(ws(tag("LAYER")), tstring))),
            ),
            |n| PinAntenna::MaxCutCar(n),
        ),
    ))(input)
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

#[cfg(test)]
mod tests {

    use crate::def_parser::pin_parser::*;
    use std::io::Read;

    #[test]
    fn test_pin_section() {
        let mut input_def = std::fs::File::open("tests/pin_test.def").unwrap();
        let mut data = String::new();
        input_def.read_to_string(&mut data).unwrap();
        let result = pin_section(&data).unwrap();

        let pin_section = result.1;

        let num = pin_section.0;
        let pins = pin_section.1;

        let pin_1_feature = (
            // true,               // Whether special
            // Some(0),            // direction
            // Some("power1 VDD"), // NetExpre
            // Some("P1"),         // PowerPin name
            // Some("P2"),         // GroundPin name
            // Some(6),            // pin mode
            // vec![
            //     (
            //         vec![
            //             PortElem::Layer(("M2", None, ((0, 0), (30, 135)))),
            //             PortElem::Via(("VIAGEN12_0", (0, 100))),
            //         ],
            //         1,
            //         (45, -2160),
            //         0,
            //     ),
            //     (
            //         vec![
            //             PortElem::Layer(("M1", None, ((0, 0), (30, 135)))),
            //             PortElem::Via(("M1_M2", (100, 0))),
            //         ],
            //         2,
            //         (0, -100),
            //         0,
            //     ),
            //     (
            //         vec![PortElem::Layer(("M3", None, ((0, 0), (30, 135))))],
            //         0,
            //         (1000, -1000),
            //         0,
            //     ),
            // ],
            vec![
                PinAntenna::PartialMetalArea((5, Some("METAL1"))),
                PinAntenna::PartialMetalArea((5, Some("M2"))),
                PinAntenna::PartialMetalSideArea((10, Some("METAL1"))),
                PinAntenna::PartialMetalSideArea((10, Some("M2"))),
                PinAntenna::PartialCutArea((35, Some("V1"))),
                PinAntenna::PartialCutArea((35, Some("V2"))),
                PinAntenna::DiffArea((20, Some("M1"))),
                PinAntenna::DiffArea((20, Some("M2"))),
                PinAntenna::Model(Some(0)),
                PinAntenna::GateArea((15, Some("M1"))),
                PinAntenna::GateArea((15, Some("M2"))),
                PinAntenna::MaxAreaCar((25, "M2")),
                PinAntenna::MaxSideAreaCar((30, "M1")),
                PinAntenna::MaxCutCar((40, "M1")),
            ],
        );

        assert_eq!(num, 11);
        assert_eq!(pins, vec![(("P0", "N0"), pin_1_feature)]);
    }
}
