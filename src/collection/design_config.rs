use nom::combinator::opt;
use nom::sequence::tuple;
use nom::IResult;

use super::def_types::DesignConfig;
use super::design_parser::{
    design_name, die_area, gcellgrid_list, prop_def_section, row_rule_def_list, technology_name,
    tracks_rule_list, units,
};
use super::via_parser::via_section;

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
        via_section,
    ))(input)
}
