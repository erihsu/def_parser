// common scope
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;

use crate::def_parser::base::ws;

pub fn orient_encode(input: &str) -> IResult<&str, i32> {
    alt((
        map(tag("N"), |_| 0),
        map(tag("W"), |_| 1),
        map(tag("S"), |_| 2),
        map(tag("E"), |_| 3),
        map(tag("FN"), |_| 4),
        map(tag("FW"), |_| 5),
        map(tag("FS"), |_| 6),
        map(tag("FE"), |_| 7),
    ))(input)
}

// source type must be preceded with "SOURCE" keyword
pub fn source_type_encode(input: &str) -> IResult<&str, i32> {
    preceded(
        ws(tag("SOURCE")),
        alt((
            map(tag("DIST"), |_| 0),
            map(tag("NETLIST"), |_| 1),
            map(tag("TIMING"), |_| 2),
            map(tag("USER"), |_| 3),
            map(tag("TEST"), |_| 4),
        )),
    )(input)
}

// shape type used in snet parser
pub fn snet_shape_encode(input: &str) -> IResult<&str, i32> {
    preceded(
        ws(tag("SHAPE")),
        alt((
            map(tag("RING"), |_| 0),
            map(tag("PADRING"), |_| 1),
            map(tag("BLOCKRING"), |_| 2),
            map(tag("STRIPE"), |_| 3),
            map(tag("FOLLOWPIN"), |_| 4),
            map(tag("IOWIRE"), |_| 5),
            map(tag("COREWIRE"), |_| 6),
            map(tag("BLOCKWIRE"), |_| 7),
            map(tag("BLOCKAGEWIRE"), |_| 8),
            map(tag("FILLWIRE"), |_| 9),
            map(tag("FILLWIREOPC"), |_| 10),
            map(tag("DRCFILL"), |_| 11),
        )),
    )(input)
}

// compatible in net and snet
pub fn use_mode_encode(input: &str) -> IResult<&str, i32> {
    preceded(
        ws(tag("USE")),
        alt((
            map(tag("ANALOG"), |_| 0),
            map(tag("CLOCK"), |_| 1),
            map(tag("GROUND"), |_| 2),
            map(tag("POWER"), |_| 3),
            map(tag("RESET"), |_| 4),
            map(tag("SCAN"), |_| 5),
            map(tag("SIGNAL"), |_| 6),
            map(tag("TIEOFF"), |_| 7),
        )),
    )(input)
}

// compatible in net and snet
pub fn net_pattern_encode(input: &str) -> IResult<&str, i32> {
    preceded(
        ws(tag("PATTERN")),
        alt((
            map(tag("BALANCED"), |_| 0),
            map(tag("STEINER"), |_| 1),
            map(tag("TRUNK"), |_| 2),
            map(tag("WIREDLOGIC"), |_| 3),
        )),
    )(input)
}

pub fn snet_global_attribute_encode(input: &str) -> IResult<&str, i32> {
    alt((
        map(tag("COVER"), |_| 0),
        map(tag("FIXED"), |_| 1),
        map(tag("ROUTED"), |_| 2),
        map(tag("SHIELD"), |_| 3),
    ))(input)
}

pub fn net_global_attribute_encode(input: &str) -> IResult<&str, i32> {
    alt((
        map(tag("COVER"), |_| 0),
        map(tag("FIXED"), |_| 1),
        map(tag("ROUTED"), |_| 2),
        map(tag("NOSHIELD"), |_| 3),
    ))(input)
}

pub fn pin_location_attribute_encode(input: &str) -> IResult<&str, i32> {
    alt((
        map(tag("PLACED"), |_| 0),
        map(tag("FIXED"), |_| 1),
        map(tag("COVER"), |_| 2),
    ))(input)
}

pub fn pin_direction_encode(input: &str) -> IResult<&str, i32> {
    preceded(
        ws(tag("DIRECTION")),
        alt((
            map(tag("INPUT"), |_| 0),
            map(tag("OUTPUT"), |_| 1),
            map(tag("INOUT"), |_| 2),
            map(tag("FEEDTHRU"), |_| 3),
        )),
    )(input)
}

pub fn pin_antenna_model_encode(input: &str) -> IResult<&str, i32> {
    alt((
        map(tag("OXIDE1"), |_| 0),
        map(tag("OXIDE2"), |_| 1),
        map(tag("OXIDE3"), |_| 2),
        map(tag("OXIDE4"), |_| 3),
    ))(input)
}

pub fn region_type_encode(input: &str) -> IResult<&str, i32> {
    preceded(
        ws(tag("TYPE")),
        alt((map(tag("FENCE"), |_| 0), map(tag("GUIDE"), |_| 1))),
    )(input)
}
