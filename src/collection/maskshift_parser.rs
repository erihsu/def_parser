// nom
use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::IResult;

// def
use crate::def_parser::base::{tstring, ws};

pub fn maskshift_section(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(tag("COMPONENTMASKSHIFT"), many1(tstring), ws(tag(";")))(input)
}

#[cfg(test)]
mod tests {
    use crate::def_parser::maskshift_parser::*;
    use std::io::Read;

    #[test]
    fn test_maskshift_section() {
        let mut input_def = std::fs::File::open("tests/maskshift_test.def").unwrap();
        let mut data = String::new();
        input_def.read_to_string(&mut data).unwrap();
        let maskshift = maskshift_section(&data).unwrap();
        assert_eq!(
            maskshift,("",vec!["M3","M2","V1","M1"])
        );
    }
}
