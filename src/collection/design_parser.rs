// nom
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::{map, opt};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;

// def
use super::base::{float, number, qstring, tstring, ws};
use super::common::{pt_list, x_or_y};
use super::encoder::orient_encode;

pub fn divider_char(input: &str) -> IResult<&str, &str> {
    delimited(
        ws(tag("DIVIDERCHAR")),
        alt((ws(tag("/")), ws(tag("\\")), ws(tag("%")), ws(tag("$")))),
        ws(tag(";")),
    )(input)
}
pub fn busbit_chars(input: &str) -> IResult<&str, &str> {
    delimited(
        ws(tag("BUSBITCHARS")),
        alt((ws(tag("[]")), ws(tag("{}")), ws(tag("<>")))),
        ws(tag(";")),
    )(input)
}

// parse version number
pub fn version_num(
    input: &str,
) -> IResult<
    &str,
    f64, // version number
> {
    delimited(ws(tag("VERSION")), float, ws(tag(";")))(input)
}

// parse namecase sensitivity
// Return: if namecase sensitivity
//         data_type: bool
// pub fn names_case_sensitivity(
//     input: &str,
// ) -> IResult<
//     &str,
//     bool, // if namecase sensitivity
// > {
//     map(
//         delimited(ws(tag("NAMESCASESENSITIVE")), on_or_off, ws(tag(";"))),
//         |s: &str| match s {
//             "ON" => true,
//             _ => false,
//         },
//     )(input)
// }

// parse design name
// Return: design name
pub fn design_name(
    input: &str,
) -> IResult<
    &str,
    &str, // design name
> {
    ws(delimited(ws(tag("DESIGN")), tstring, ws(tag(";"))))(input)
}

// parse technology name
pub fn technology_name(
    input: &str,
) -> IResult<
    &str,
    &str, // technology name
> {
    ws(delimited(ws(tag("TECHNOLOGY")), tstring, ws(tag(";"))))(input)
}

// parse units
pub fn units(
    input: &str,
) -> IResult<
    &str,
    i32, // units of def scope
> {
    delimited(ws(tag("UNITS DISTANCE MICRONS")), number, ws(tag(";")))(input)
}

// parse die_area
pub fn die_area(
    input: &str,
) -> IResult<
    &str,
    Vec<(i32, i32)>, // die area
> {
    delimited(ws(tag("DIEAREA")), pt_list, ws(tag(";")))(input)
}

// parse property definitions
pub fn prop_def_section(
    input: &str,
) -> IResult<
    &str,
    Vec<(
        &str, // object type of property. ie, design, region, group, component,...
        &str, // property name
        (
            char, // data type of property
            bool, // whether has range
            bool, // whether has number
            bool, // whether has name_map_string
            &str, // string data if belongs to 'S' data type
            f64,  // left boarder of range if has range
            f64,  // right boarder of range if has range
            f64,  // data if has number
        ),
    )>,
> {
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
pub fn row_rule_def_list(
    input: &str,
) -> IResult<
    &str,
    Vec<(
        (
            &str, // name of row rule
            &str, // row rule type
            (
                i32, //  the origin of rule scope along axis X
                i32, // the origin of rule scope along axis Y
            ),
            i32, // orient of row rule
            (
                i32, // step number along axis X
                i32, // step number along axis Y
                i32, // step size along axis X
                i32, // step size along axis Y
            ),
        ),
        (
            Vec<&str>, // property name of row rule
            Vec<&str>, // property value(String type) of row rule
            Vec<f64>,  // property value(real type) of row rule
            i32,       // number of property of row rule
        ),
    )>,
> {
    many1(tuple((
        tuple((
            preceded(ws(tag("ROW")), tstring),
            row_type,
            pair(number, number),
            orient_encode,
            tuple((
                preceded(ws(tag("DO")), number),
                preceded(ws(tag("BY")), number),
                preceded(ws(tag("STEP")), number),
                number,
            )),
        )),
        terminated(row_properties, ws(tag(";"))),
    )))(input)
}

// parse gcell grid
// Return:
//    vector of (x_or_y, start,numColumns+1/numRows+1, steps)
pub fn gcellgrid_list(
    input: &str,
) -> IResult<
    &str,
    Vec<(
        char, // axis. 'X' or 'Y'
        i32,  // the origin of gcell grid along axis X/Y
        i32,  // the step number of gcell grid
        i32,  // the step size of gcell grid
    )>,
> {
    many1(tuple((
        preceded(ws(tag("GCELLGRID")), x_or_y),
        number,
        preceded(ws(tag("DO")), number),
        delimited(ws(tag("STEP")), number, ws(tag(";"))),
    )))(input)
}

pub fn tracks_rule_list(
    input: &str,
) -> IResult<
    &str,
    Vec<(
        (
            char, // axis. 'X' or 'Y'
            i32,  // the origin of gcell grid along axis X/Y
            i32,  // the step number of track
            i32,  // the step size of track
        ),
        (
            Option<i32>,       // mask number of the track. optional
            Option<bool>,      // whether the samemask. optional
            Option<Vec<&str>>, // the metal layer of track
        ),
    )>,
> {
    many1(tuple((
        tuple((
            preceded(ws(tag("TRACKS")), x_or_y),
            number,
            preceded(ws(tag("DO")), number),
            preceded(ws(tag("STEP")), number),
        )),
        terminated(track_option, ws(tag(";"))),
    )))(input)
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

// return tuple includes information that
//                      (data_type, has_range, has_number,has_name_map_string,
//                       string_data,range_left,range_right,num_data)
fn property_type_and_val(
    input: &str,
) -> IResult<&str, (char, bool, bool, bool, &str, f64, f64, f64)> {
    alt((
        string_prop,
        integer_range_prop,
        integer_prop,
        real_range_prop,
        real_prop,
    ))(input)
}

fn string_prop(input: &str) -> IResult<&str, (char, bool, bool, bool, &str, f64, f64, f64)> {
    map(
        preceded(ws(tag("STRING")), opt(qstring)),
        |res: Option<&str>| {
            let mut has_map_string = false;
            let mut str_data = "";
            match res {
                Some(s) => {
                    has_map_string = true;
                    str_data = s;
                }
                None => {}
            }
            ('S', false, false, has_map_string, str_data, 0.0, 0.0, 0.0)
        },
    )(input)
}

fn integer_prop(input: &str) -> IResult<&str, (char, bool, bool, bool, &str, f64, f64, f64)> {
    map(
        preceded(ws(tag("INTEGER")), opt(number)),
        |res: Option<i32>| {
            let mut has_number = false;
            let mut num_data = 0.0;
            match res {
                Some(s) => {
                    has_number = true;
                    num_data = s as f64;
                }
                None => {}
            }
            ('I', false, has_number, false, "", 0.0, 0.0, num_data)
        },
    )(input)
}

fn real_prop(input: &str) -> IResult<&str, (char, bool, bool, bool, &str, f64, f64, f64)> {
    map(preceded(ws(tag("REAL")), opt(float)), |res: Option<f64>| {
        let mut has_number = false;
        let mut num_data = 0.0;
        match res {
            Some(s) => {
                has_number = true;
                num_data = s;
            }
            None => {}
        }
        ('R', false, has_number, false, "", 0.0, 0.0, num_data)
    })(input)
}

fn integer_range_prop(input: &str) -> IResult<&str, (char, bool, bool, bool, &str, f64, f64, f64)> {
    map(
        preceded(
            ws(tag("INTEGER RANGE")),
            many1(delimited(space0, number, space0)),
        ),
        |res: Vec<i32>| {
            let mut has_number = false;
            let mut num_data = 0.0;
            if res.len() == 3 {
                has_number = true;
                num_data = res[2] as f64;
            }
            (
                'I',
                true,
                has_number,
                false,
                "",
                res[0] as f64,
                res[1] as f64,
                num_data,
            )
        },
    )(input)
}

fn real_range_prop(input: &str) -> IResult<&str, (char, bool, bool, bool, &str, f64, f64, f64)> {
    map(
        preceded(
            ws(tag("REAL RANGE")),
            many1(delimited(space0, float, space0)),
        ),
        |res: Vec<f64>| {
            let mut has_number = false;
            let mut num_data = 0.0;

            if res.len() == 3 {
                has_number = true;
                num_data = res[2];
            }

            ('R', true, has_number, false, "", res[0], res[1], num_data)
        },
    )(input)
}

// return property_name, property_value and property_dvalue and the number of properties
fn row_properties(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>, Vec<f64>, i32)> {
    map(many0(row_prop), |res: Vec<(&str, &str, f64)>| {
        let mut prop_name = Vec::new();
        let mut prop_value = Vec::new();
        let mut prop_dvalue = Vec::new();
        let mut num_properties = 0;
        for s in res {
            prop_name.push(s.0);
            prop_value.push(s.1);
            prop_dvalue.push(s.2);
            num_properties += 1;
        }
        (prop_name, prop_value, prop_dvalue, num_properties)
    })(input)
}

// Return optional data includes three data that prop_name, prop_value and prop_dvalue
fn row_prop(input: &str) -> IResult<&str, (&str, &str, f64)> {
    preceded(
        ws(tag("+ PROPERTY")),
        alt((
            row_string_prop,
            row_real_prop,
            row_integer_prop,
            row_real_range_prop,
            row_integer_range_prop,
        )),
    )(input)
}

fn row_string_prop(input: &str) -> IResult<&str, (&str, &str, f64)> {
    map(pair(tstring, qstring), |res: (&str, &str)| {
        (res.0, res.1, 0.0)
    })(input)
}

fn row_integer_prop(input: &str) -> IResult<&str, (&str, &str, f64)> {
    map(pair(tstring, number), |res: (&str, i32)| {
        (res.0, "", res.1 as f64)
    })(input)
}

fn row_real_prop(input: &str) -> IResult<&str, (&str, &str, f64)> {
    map(pair(tstring, float), |res: (&str, f64)| (res.0, "", res.1))(input)
}

fn row_integer_range_prop(input: &str) -> IResult<&str, (&str, &str, f64)> {
    map(pair(tstring, number), |res: (&str, i32)| {
        (res.0, "", res.1 as f64)
    })(input)
}

fn row_real_range_prop(input: &str) -> IResult<&str, (&str, &str, f64)> {
    map(pair(tstring, float), |res: (&str, f64)| (res.0, "", res.1))(input)
}

fn row_type(input: &str) -> IResult<&str, &str> {
    alt((ws(tag("CORE")), ws(tag("ARRAYSITE"))))(input)
}

fn track_option(input: &str) -> IResult<&str, (Option<i32>, Option<bool>, Option<Vec<&str>>)> {
    tuple((
        opt(preceded(ws(tag("MASK")), number)),
        map(opt(ws(tag("SAMEMASK"))), |res: Option<&str>| match res {
            Some(_) => Some(true),
            None => None,
        }),
        opt(preceded(ws(tag("LAYER")), many1(tstring))),
    ))(input)
}

// #[cfg(test)]
// mod tests {
//     use crate::def_parser::design_parser::*;
//     #[test]
//     fn test_version() {
//         assert_eq!(version_num("VERSION 5.8 ; \n").unwrap(), ("", 5.8));
//     }

//     #[test]
//     fn test_sensitivity() {
//         assert_eq!(
//             names_case_sensitivity("NAMESCASESENSITIVE ON ;\n").unwrap(),
//             ("", true)
//         );
//     }

//     #[test]
//     fn test_design_name() {
//         assert_eq!(
//             design_name("DESIGN my_design ; \n").unwrap(),
//             ("", "my_design")
//         );
//     }

//     #[test]
//     fn test_technology() {
//         assert_eq!(
//             technology_name("TECHNOLOGY smic40_ll ;").unwrap(),
//             ("", "smic40_ll")
//         );
//     }
//     #[test]
//     fn test_units() {
//         assert_eq!(
//             units("UNITS DISTANCE MICRONS 1000 ;\n").unwrap(),
//             ("", 1000)
//         );
//     }
//     #[test]
//     fn test_row_rule_def_list() {
//         assert_eq!(
//             row_rule_def_list(
//                 "ROW ROW_1 CORE 1000 1000 N DO 100 BY 1 STEP 700 0\n
//                   + PROPERTY strprop \"aString\" \n
//                   + PROPERTY intprop 1 \n
//                   + PROPERTY realprop 1.1 \n
//                   + PROPERTY intrangeprop 25 \n
//                   + PROPERTY realrangeprop 25.25 ; \n
//                 ROW ROW_2 CORE 1000 2000 S DO 100 BY 1 STEP 700 0 ; \n
//                 ROW ROW_array2 ARRAYSITE 10000 17000 S DO 100 BY 1 STEP 16000 0 ;\n
//                 ROW ROW_VERT_1 CORE -10000 -10000 N DO 1 BY 10 STEP 0 8400 ;\n"
//             )
//             .unwrap(),
//             (
//                 "",
//                 vec![
//                     (
//                         ("ROW_1", "CORE", (1000, 1000), 0, (100, 1, 700, 0)),
//                         (
//                             vec![
//                                 "strprop",
//                                 "intprop",
//                                 "realprop",
//                                 "intrangeprop",
//                                 "realrangeprop"
//                             ],
//                             vec!["\"aString\"", "", "", "", ""],
//                             vec![0.0, 1.0, 1.1, 25.0, 25.25],
//                             5
//                         )
//                     ),
//                     (
//                         ("ROW_2", "CORE", (1000, 2000), 2, (100, 1, 700, 0)),
//                         (vec![], vec![], vec![], 0)
//                     ),
//                     (
//                         (
//                             "ROW_array2",
//                             "ARRAYSITE",
//                             (10000, 17000),
//                             2,
//                             (100, 1, 16000, 0)
//                         ),
//                         (vec![], vec![], vec![], 0)
//                     ),
//                     (
//                         ("ROW_VERT_1", "CORE", (-10000, -10000), 0, (1, 10, 0, 8400)),
//                         (vec![], vec![], vec![], 0)
//                     ),
//                 ]
//             )
//         );
//     }

//     #[test]
//     fn test_prop_def_section() {
//         assert_eq!(
//             prop_def_section(
//                 "PROPERTYDEFINITIONS\n
//             DESIGN strprop STRING \"aString\" ;\n
//             REGION realprop REAL ;\n
//             GROUP intprop INTEGER ;\n
//             COMPONENT intrangeprop INTEGER RANGE 1 100 ; \n
//             NET intprop INTEGER ; \n
//             END PROPERTYDEFINITIONS"
//             )
//             .unwrap(),
//             (
//                 "",
//                 vec![
//                     (
//                         "DESIGN",
//                         "strprop",
//                         ('S', false, false, true, "\"aString\"", 0.0, 0.0, 0.0)
//                     ),
//                     (
//                         "REGION",
//                         "realprop",
//                         ('R', false, false, false, "", 0.0, 0.0, 0.0)
//                     ),
//                     (
//                         "GROUP",
//                         "intprop",
//                         ('I', false, false, false, "", 0.0, 0.0, 0.0)
//                     ),
//                     (
//                         "COMPONENT",
//                         "intrangeprop",
//                         ('I', true, false, false, "", 1.0, 100.0, 0.0)
//                     ),
//                     (
//                         "NET",
//                         "intprop",
//                         ('I', false, false, false, "", 0.0, 0.0, 0.0)
//                     )
//                 ]
//             )
//         );
//     }

//     #[test]
//     fn test_tracks_rule_list() {
//         assert_eq!(
//             tracks_rule_list(
//                 "TRACKS Y 52 DO 857 STEP 104 MASK 1 ;\n TRACKS Y 52 DO 857 STEP 104 MASK 1 SAMEMASK LAYER M1 M2 ;"
//             )
//             .unwrap(),
//             (
//                 "",
//                 vec![
//                     (('Y', 52, 857, 104), (Some(1), None, None)),
//                     (
//                         ('Y', 52, 857, 104),
//                         (Some(1), Some(true), Some(vec!["M1", "M2"]))
//                     )
//                 ]
//             )
//         );
//     }
//     #[test]
//     fn test_gcellgrid() {
//         assert_eq!(
//             gcellgrid_list("GCELLGRID X 0 DO 100 STEP 600 ; \n GCELLGRID Y 10 DO 120 STEP 400 ;\n")
//                 .unwrap(),
//             ("", vec![('X', 0, 100, 600), ('Y', 10, 120, 400)])
//         );
//     }
//     #[test]
//     fn test_die_area() {
//         assert_eq!(
//             die_area(
//                 "DIEAREA ( -190000 -120000 ) ( -190000 350000 ) ( 190000 350000 ) \n
//         ( 190000 190000 ) ( 190360 190000 ) ( 190360 -120000 ) ;"
//             )
//             .unwrap(),
//             (
//                 "",
//                 vec![
//                     (-190000, -120000),
//                     (-190000, 350000),
//                     (190000, 350000),
//                     (190000, 190000),
//                     (190360, 190000),
//                     (190360, -120000)
//                 ]
//             )
//         );
//     }
// }
