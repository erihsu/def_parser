use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

use super::base::{float, number, tstring, ws};
use super::common::{properties, pt_list, x_or_y};
use super::def_types::{DesignConfig, GcellGrid, PropDef, Row, Track};
use super::encoder::orient_encode;

pub fn design_config(input: &str) -> IResult<&str, DesignConfig> {
    tuple((
        design_name,
        opt(technology_name),
        opt(units),
        // HISTORY not supported
        opt(prop_def_section),
        opt(die_area),
        opt(row_rule_def_list),
        opt(tracks_rule_list),
        opt(gcellgrid_list),
    ))(input)
}

// parse design name
// Return: design name
fn design_name(
    input: &str,
) -> IResult<
    &str,
    &str, // design name
> {
    ws(delimited(ws(tag("DESIGN")), tstring, ws(tag(";"))))(input)
}

// parse technology name
fn technology_name(
    input: &str,
) -> IResult<
    &str,
    &str, // technology name
> {
    ws(delimited(ws(tag("TECHNOLOGY")), tstring, ws(tag(";"))))(input)
}

// parse units
fn units(
    input: &str,
) -> IResult<
    &str,
    i32, // units of def scope
> {
    delimited(ws(tag("UNITS DISTANCE MICRONS")), number, ws(tag(";")))(input)
}

// parse die_area
fn die_area(
    input: &str,
) -> IResult<
    &str,
    Vec<(i32, i32)>, // die area
> {
    delimited(ws(tag("DIEAREA")), pt_list, ws(tag(";")))(input)
}

// parse property definitions
fn prop_def_section(input: &str) -> IResult<&str, Vec<PropDef>> {
    delimited(
        ws(tag("PROPERTYDEFINITIONS")),
        many1(tuple((
            object_type,
            tstring,
            terminated(property_type_and_val, ws(tag(";"))),
        ))),
        ws(tag("END PROPERTYDEFINITIONS")),
    )(input)
}

// parse row rule definition
fn row_rule_def_list(input: &str) -> IResult<&str, Vec<Row>> {
    many1(delimited(
        ws(tag("ROW")),
        tuple((
            tstring,
            row_type,
            number,
            number,
            orient_encode,
            preceded(ws(tag("DO")), number),
            preceded(ws(tag("BY")), number),
            preceded(ws(tag("STEP")), number),
            number,
            opt(properties),
        )),
        ws(tag(";")),
    ))(input)
}

// parse gcell grid
// Return:
//    vector of (x_or_y, start,numColumns+1/numRows+1, steps)
fn gcellgrid_list(input: &str) -> IResult<&str, Vec<GcellGrid>> {
    many1(delimited(
        ws(tag("GCELLGRID")),
        tuple((
            x_or_y,
            number,
            preceded(ws(tag("DO")), number),
            preceded(ws(tag("STEP")), number),
        )),
        ws(tag(";")),
    ))(input)
}

fn tracks_rule_list(input: &str) -> IResult<&str, Vec<Track>> {
    many1(delimited(
        ws(tag("TRACKS")),
        tuple((
            x_or_y,
            number,
            preceded(ws(tag("DO")), number),
            preceded(ws(tag("STEP")), number),
            opt(preceded(ws(tag("LAYER")), many1(tstring))),
        )),
        ws(tag(";")),
    ))(input)
}

fn object_type(input: &str) -> IResult<&str, &str> {
    alt((
        ws(tag("DESIGN")),
        ws(tag("REGION")),
        ws(tag("GROUP")),
        ws(tag("COMPONENTPIN")),
        ws(tag("COMPONENT")),
        ws(tag("NET")),
        ws(tag("SPECIALNET")),
        ws(tag("ROW")),
        ws(tag("NONDEFAULTRULE")),
    ))(input)
}

fn row_type(input: &str) -> IResult<&str, &str> {
    alt((ws(tag("CORE")), ws(tag("ARRAYSITE"))))(input)
}

// return tuple includes information that
//                      (data_type, has_range, has_number,has_name_map_string,
//                       string_data,range_left,range_right,num_data)
fn property_type_and_val(
    input: &str,
) -> IResult<
    &str,
    (
        char, // data type of property
        Option<&str>,
        Option<(i32, Option<(i32, i32)>)>,
        Option<(f64, Option<(f64, f64)>)>,
    ),
> {
    alt((
        map(preceded(ws(tag("STRING")), tstring), |n| {
            ('S', Some(n), None, None)
        }),
        map(preceded(ws(tag("INTEGER")), number), |n| {
            ('I', None, Some((n, None)), None)
        }),
        map(preceded(ws(tag("REAL")), float), |n| {
            ('R', None, None, Some((n, None)))
        }),
        map(
            preceded(ws(tag("INTEGER RANGE")), tuple((number, number, number))),
            |n| ('R', None, Some((n.0, Some((n.1, n.2)))), None),
        ),
        map(
            preceded(ws(tag("REAL RANGE")), tuple((float, float, float))),
            |n| ('R', None, None, Some((n.0, Some((n.1, n.2))))),
        ),
    ))(input)
}
