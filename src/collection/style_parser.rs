// nom
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;

// def
use crate::def_parser::base::{number, ws};
use crate::def_parser::common::pt_list;
use crate::def_parser::def_types::Style;

pub fn style_section(
    input: &str,
) -> IResult<
    &str,
    (
        i32, // total number of styles
        Vec<Style>,
    ),
> {
    delimited(
        tag("STYLES"),
        tuple((
            terminated(number, ws(tag(";"))), // total number of styles
            many0(style_member),
        )),
        tag("END STYLES"),
    )(input)
}

fn style_member(input: &str) -> IResult<&str, Style> {
    delimited(tag("- STYLE"), tuple((number, pt_list)), ws(tag(";")))(input)
}

#[cfg(test)]
mod tests {
    use crate::def_parser::def_types::*;
    use crate::def_parser::style_parser::*;
    use std::io::Read;

    #[test]
    fn test_style_section() {
        let mut input_def = std::fs::File::open("tests/style_test.def").unwrap();
        let mut data = String::new();
        input_def.read_to_string(&mut data).unwrap();
        let result = style_section(&data).unwrap();

        let style_section = result.1;

        let num = style_section.0;
        let styles = style_section.1;

        assert_eq!(num, 10);
        assert_eq!(
            styles,
            vec![
                (
                    0,
                    vec![
                        (30, 10),
                        (10, 30),
                        (-10, 30),
                        (-30, 10),
                        (-30, -10),
                        (-10, -30),
                        (10, -30),
                        (30, -10)
                    ],
                ),
                (1, vec![(25, 25), (-25, 25), (-25, -25), (25, -25)]),
                (2, vec![(50, 50), (-50, 50), (-50, -50), (50, -50)]),
                (
                    3,
                    vec![
                        (50, 21),
                        (21, 50),
                        (-21, 50),
                        (-50, 21),
                        (-50, -21),
                        (-21, -50),
                        (21, -50),
                        (50, -21)
                    ]
                ),
                (
                    4,
                    vec![
                        (-30, -20),
                        (10, -60),
                        (50, -20),
                        (50, 40),
                        (0, 40),
                        (-30, 10)
                    ]
                ),
                (5, vec![(0, 2000), (0, -2000), (0, 2000), (0, -2000)]),
                (6, vec![(-2000, 2000), (2000, -2000), (2000, -2000)]),
                (7, vec![(0, 0), (0, 1000), (1000, 0)]),
                (
                    8,
                    vec![
                        (-7500, -3110),
                        (-3110, -7500),
                        (3110, -7500),
                        (7500, -3110),
                        (7500, 7500),
                        (-7500, 7500)
                    ]
                ),
                (
                    9,
                    vec![
                        (0, -10610),
                        (7500, -3110),
                        (7500, 3110),
                        (3110, 7500),
                        (-3110, 7500),
                        (-10610, 0)
                    ]
                ),
            ]
        )
    }
}
