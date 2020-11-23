// nom
use nom::branch::{alt, permutation};
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{float, number, tstring, ws};
use crate::def_parser::common::rect_or_polygon;
use crate::def_parser::def_types::{Blockage, Geometry};

pub fn blockage_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // numBlockage
        Vec<Blockage>,
    ),
> {
    delimited(
        tag("BLOCKAGES"),
        tuple((
            terminated(number, ws(tag(";"))), // blockage num
            many0(blockage_member),
        )),
        tag("END BLOCKAGES"),
    )(input)
}

fn blockage_member(input: &str) -> IResult<&str, Blockage> {
    delimited(
        tag("-"),
        alt((
            map(
                tuple((
                    preceded(ws(tag("LAYER")), tstring),
                    layer_blockage_rule,
                    many0(rect_or_polygon),
                )),
                |res: (
                    &str,
                    (
                        bool,
                        bool,
                        bool,
                        bool,
                        Option<i32>,
                        Option<&str>,
                        Option<i32>,
                    ),
                    Vec<Geometry>,
                )| { Blockage::Layer(res) },
            ),
            map(
                tuple((
                    preceded(ws(tag("PLACEMENT")), placement_blockage_rule),
                    many0(rect_or_polygon),
                )),
                |res: ((bool, Option<f64>, bool, Option<&str>), Vec<Geometry>)| {
                    Blockage::Placement(res)
                },
            ),
        )),
        ws(tag(";")),
    )(input)
}

fn layer_blockage_rule(
    input: &str,
) -> IResult<
    &str,
    (
        bool, // SLOTS. Whether creates a blockage on the specified layer where slots cannot be placed.
        bool, // PUSHDOWN. Specifies that whether the blockage was pushed down into the block from the top level of the design.
        bool, // FILLS. Whether creates a blockage on the specified layer where metal fills cannot be placed.
        bool, // EXCEPTPGNET. Indicates that whether the blockage only blocks signal net routing, and does not block power or ground net routing.
        Option<i32>, // SPACING or DESIGNRULEWIDTH. minimum spacing allowed between the blockage and any other routing shape
        Option<&str>, // COMPONENT. component with which to associate a blockage.
        Option<i32>, // MASK.
    ),
> {
    tuple((
        map(opt(ws(tag("+ SLOTS"))), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        map(opt(ws(tag("+ PUSHDOWN"))), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        map(opt(ws(tag("+ FILLS"))), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        map(
            opt(ws(tag("+ EXCEPTPGNET"))),
            |res: Option<&str>| match res {
                Some(_) => true,
                None => false,
            },
        ),
        opt(alt((
            preceded(ws(tag("+ SPACING")), number),
            preceded(ws(tag("+ DESIGNRULEWIDTH")), number),
        ))),
        opt(preceded(ws(tag("+ COMPONENT")), tstring)),
        opt(preceded(ws(tag("+ MASK")), number)),
    ))(input)
}

fn placement_blockage_rule(
    input: &str,
) -> IResult<
    &str,
    (
        bool, //SOFT. Indicates that whether the initial placement should not use the area, but later phases, such as timing optimization or clock tree synthesis, can use the blockage area.
        Option<f64>, // PARTIAL. Indicates that the initial placement should not use more than partial percentage of the blockage area for standard cells.
        bool, //PUSHDOWN. Specifies that whether the blockage was pushed down into the block from the top level of the design.
        Option<&str>, // COMPONENT. component with which to associate a blockage.
    ),
> {
    permutation((
        map(opt(ws(tag("+ SOFT"))), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        opt(preceded(ws(tag("+ PARTIAL")), float)),
        map(opt(tag("+ PUSHDOWN")), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        opt(preceded(ws(tag("+ COMPONENT")), tstring)),
    ))(input)
}

#[cfg(test)]
mod tests {
    use crate::def_parser::blockage_parser::*;
    use crate::def_parser::def_types::*;
    use std::io::Read;

    #[test]
    fn test_blockage_section() {
        let mut input_def = std::fs::File::open("tests/blockage_test.def").unwrap();
        let mut data = String::new();
        input_def.read_to_string(&mut data).unwrap();
        let result = blockage_section(&data).unwrap();

        let blockage_section = result.1;

        let num = blockage_section.0;
        let blockages = blockage_section.1;

        assert_eq!(num, 8);
        assert_eq!(
            blockages,
            vec![
                Blockage::Layer((
                    "METAL1",
                    (false, false, false, false, None, None, Some(1)),
                    vec![Geometry::Rect(((60, 70), (80, 90)))]
                )),
                Blockage::Layer((
                    "M2",
                    (true, true, false, true, None, Some("I1"), Some(3)),
                    vec![Geometry::Polygon(vec![
                        (100, 100),
                        (100, 200),
                        (150, 200),
                        (150, 150),
                        (200, 150),
                        (200, 100)
                    ])]
                )),
                Blockage::Layer((
                    "M2",
                    (true, false, false, false, None, None, Some(2)),
                    vec![Geometry::Rect(((10, 20), (40, 50)))]
                )),
                Blockage::Layer((
                    "M1",
                    (false, true, false, false, Some(3), None, Some(1)),
                    vec![Geometry::Rect(((50, 30), (55, 40)))]
                )),
                Blockage::Layer((
                    "M1",
                    (false, false, false, true, Some(45), None, Some(1)),
                    vec![Geometry::Rect(((50, 30), (55, 40)))]
                )),
                Blockage::Placement((
                    (false, None, false, None),
                    vec![
                        Geometry::Rect(((-15, 0), (0, 20))),
                        Geometry::Rect(((-10, -15), (50, 0)))
                    ]
                )),
                Blockage::Placement((
                    (false, Some(0.4), true, Some("I1")),
                    vec![
                        Geometry::Rect(((-10, 0), (0, 20))),
                        Geometry::Rect(((-10, -5), (50, 0)))
                    ]
                )),
                Blockage::Placement((
                    (true, None, false, None),
                    vec![Geometry::Rect(((50, 30), (55, 40)))]
                )),
            ]
        );
    }
}
