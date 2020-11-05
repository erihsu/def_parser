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
                    (Option<i32>, Option<&str>, bool, bool, bool, bool, i32),
                    Vec<Geometry>,
                )| { Blockage::Layer(res) },
            ),
            map(
                tuple((
                    preceded(ws(tag("PLACEMENT")), placement_blockage_rule),
                    many0(rect_or_polygon),
                )),
                |res: ((Option<&str>, bool, bool, Option<f64>), Vec<Geometry>)| {
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
        Option<i32>, // SPACING or DESIGNRULEWIDTH. minimum spacing allowed between the blockage and any other routing shape
        Option<&str>, // COMPONENT. component with which to associate a blockage.
        bool, // SLOTS. Whether creates a blockage on the specified layer where slots cannot be placed.
        bool, // FILLS. Whether creates a blockage on the specified layer where metal fills cannot be placed.
        bool, // PUSHDOWN. Specifies that whether the blockage was pushed down into the block from the top level of the design.
        bool, // EXCEPTPGNET. Indicates that whether the blockage only blocks signal net routing, and does not block power or ground net routing.
        i32,  // MASK.
    ),
> {
    permutation((
        opt(preceded(
            alt((tag("+ SPACING"), tag("+ DESIGNRULEWIDTH"))),
            number,
        )),
        opt(preceded(tag("+ COMPONENT"), tstring)),
        map(opt(tag("+ SLOTS")), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        map(opt(tag("+ FILLS")), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        map(opt(tag("+ PUSHDOWN")), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        map(opt(tag("+ EXCEPTPGNET")), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        preceded(tag("+ MASK"), number),
    ))(input)
}

fn placement_blockage_rule(
    input: &str,
) -> IResult<
    &str,
    (
        Option<&str>, // COMPONENT. component with which to associate a blockage.
        bool, //PUSHDOWN. Specifies that whether the blockage was pushed down into the block from the top level of the design.
        bool, //SOFT. Indicates that whether the initial placement should not use the area, but later phases, such as timing optimization or clock tree synthesis, can use the blockage area.
        Option<f64>, // PARTIAL. Indicates that the initial placement should not use more than partial percentage of the blockage area for standard cells.
    ),
> {
    permutation((
        opt(preceded(tag("+ COMPONENT"), tstring)),
        map(opt(tag("+ PUSHDOWN")), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        map(opt(tag("+ SOFT")), |res: Option<&str>| match res {
            Some(_) => true,
            None => false,
        }),
        opt(preceded(tag("+ PARTIAL"), float)),
    ))(input)
}
