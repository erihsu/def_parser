// common scope

pub fn orient_encode(input: &str) -> Result<i32, &str> {
    match input {
        "N" => Ok(0),
        "W" => Ok(1),
        "S" => Ok(2),
        "E" => Ok(3),
        "FN" => Ok(4),
        "FW" => Ok(5),
        "FS" => Ok(6),
        "FE" => Ok(7),
        _ => Err("Unsupported orient!"),
    }
}

pub fn source_type_encode(input: &str) -> Result<i32, &str> {
    match input {
        "DIST" => Ok(0),
        "NETLIST" => Ok(1),
        "TIMING" => Ok(2),
        "USER" => Ok(3),
        "TEST" => Ok(4),
        _ => Err("Unsupported source type"),
    }
}

pub fn snet_shape_encode(input: &str) -> Result<i32, &str> {
    match input {
        "RING" => Ok(0),
        "PADRING" => Ok(1),
        "BLOCKRING" => Ok(2),
        "STRIPE" => Ok(3),
        "FOLLOWPIN" => Ok(4),
        "IOWIRE" => Ok(5),
        "COREWIRE" => Ok(6),
        "BlOCKWIRE" => Ok(7),
        "BLOCKAGEWIRE" => Ok(8),
        "FILLWIRE" => Ok(9),
        "FILLWIREOPC" => Ok(10),
        "DRCFILL" => Ok(11),
        _ => Err("Unsupported shape type"),
    }
}

// compatible in net and snet
pub fn use_mode_encode(input: &str) -> Result<i32, &str> {
    match input {
        "ANALOG" => Ok(0),
        "CLOCK" => Ok(1),
        "GROUND" => Ok(2),
        "POWER" => Ok(3),
        "RESET" => Ok(4),
        "SCAN" => Ok(5),
        "SIGNAL" => Ok(6),
        "TIEOFF" => Ok(7),
        _ => Err("Unsupported use type!"),
    }
}

pub fn net_pattern_encode(input: &str) -> Result<i32, &str> {
    match input {
        "BALANCED" => Ok(0),
        "STEINER" => Ok(1),
        "TRUNK" => Ok(2),
        "WIREDLOGIC" => Ok(3),
        _ => Err("Unsupported pattern type!"),
    }
}

pub fn snet_global_attribute_encode(input: &str) -> Result<i32, &str> {
    match input {
        "COVER" => Ok(0),
        "FIXED" => Ok(1),
        "ROUTED" => Ok(2),
        "SHIELD" => Ok(3),
        _ => Err("Unsupported attribute for special net!"),
    }
}

pub fn net_global_attribute_encode(input: &str) -> Result<i32, &str> {
    match input {
        "COVER" => Ok(0),
        "FIXED" => Ok(1),
        "ROUTED" => Ok(2),
        "NOSHIELD" => Ok(3),
        _ => Err("Unsupported attribute for regular net!"),
    }
}

pub fn pin_location_attribute_encode(input: &str) -> Result<i32, &str> {
    match input {
        "PLACED" => Ok(0),
        "FIXED" => Ok(1),
        "COVERED" => Ok(2),
        _ => Err("Unsupported location attribute for pin!"),
    }
}

pub fn pin_direction(input: &str) -> Result<i32, &str> {
    match input {
        "INPUT" => Ok(0),
        "OUTPUT" => Ok(1),
        "INOUT" => Ok(2),
        "FEEDTHRU" => Ok(3),
        _ => Err("Unsupported direction for pin"),
    }
}

pub fn pin_antenna_model(input: &str) -> Result<i32, &str> {
    match input {
        "OXIDE1" => Ok(0),
        "OXIDE2" => Ok(1),
        "OXIDE3" => Ok(2),
        "OXIDE4" => Ok(3),
        _ => Err("Unsupported Oxide model for pin!"),
    }
}
