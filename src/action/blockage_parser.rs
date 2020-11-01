use crate::action::common_parse::{float, inline_comment, number, pt_list, tstring, ws};

use def::miscellaneous_rt::geometries::DefGeometries;
use nom::branch::{alt, permutation};
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

pub fn blockage_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32,
        Vec<(
            &str,
            (Option<i32>, Option<&str>, bool, bool, bool, bool, i32),
            Vec<DefGeometries>,
            Option<&str>,
        )>,
        Vec<(
            (Option<&str>, bool, bool, Option<f64>),
            Vec<DefGeometries>,
            Option<&str>,
        )>,
    ),
> {
    terminated(
        tuple((
            delimited(ws(tag("BLOCKAGES")), number, ws(tag(";"))),
            many1(blockage_layer),
            many1(blockage_placement),
        )),
        ws(tag("END BLOCKAGES")),
    )(input)
}

fn blockage_layer(
    input: &str,
) -> IResult<
    &str,
    (
        &str,
        (Option<i32>, Option<&str>, bool, bool, bool, bool, i32),
        Vec<DefGeometries>,
        Option<&str>,
    ),
> {
    tuple((
        preceded(ws(tag("- LAYER")), tstring),
        layer_blockage_rule,
        many1(blockage_geometries),
        preceded(ws(tag(";")), opt(inline_comment)),
    ))(input)
}

fn blockage_placement(
    input: &str,
) -> IResult<
    &str,
    (
        (Option<&str>, bool, bool, Option<f64>),
        Vec<DefGeometries>,
        Option<&str>,
    ),
> {
    tuple((
        preceded(ws(tag("- PLACEMENT")), placement_blockage_rule),
        many1(blockage_geometries),
        preceded(ws(tag(";")), opt(inline_comment)),
    ))(input)
}

fn blockage_geometries(input: &str) -> IResult<&str, DefGeometries> {
    map(
        preceded(alt((ws(tag("POLYGON")), ws(tag("RECT")))), pt_list),
        |res: Vec<(&str, &str)>| {
            let mut out = DefGeometries::new();
            let mut prev_x = 0;
            let mut prev_y = 0;
            out.num_points = res.len() as i32;
            for (pt_x, pt_y) in res {
                if pt_x == "*" {
                    out.x.push(prev_x);
                } else {
                    prev_x = pt_x.parse::<i32>().unwrap();
                    out.x.push(prev_x);
                }
                if pt_y == "*" {
                    out.y.push(prev_y);
                } else {
                    prev_y = pt_y.parse::<i32>().unwrap();
                    out.y.push(prev_y);
                }
            }
            out
        },
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
