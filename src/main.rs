use eip_plc::{add, EipPlc, EipTagType, TagResult};

fn main() {
    println!("Hello World");
    // println!("{}", add(2, 5));
    //
    // let timeout = 100;//ms
    // let mut path="protocol=ab-eip&plc=controllogix&path=1,1&gateway=192.168.1.14&name=BaseBOOL";
    // let mut tag = RawTag::new(path, timeout).unwrap();
    // let mut status = tag.read(timeout);
    // assert!(status.is_ok());
    // let value: u8 = tag.get_value(0).unwrap();
    //
    // println!("{}", value);
    //
    // path = "protocol=ab-eip&plc=controllogix&path=1,1&gateway=192.168.1.14&name=BaseREAL";
    // tag = RawTag::new(path, timeout).unwrap();
    // status = tag.read(timeout);
    // assert!(status.is_ok());
    // let real: f32 = tag.get_f32(0).unwrap();
    //
    // println!("{}", real);

    let path="protocol=ab-eip&plc=controllogix&path=1,1&gateway=192.168.1.14&name=".to_string();
    let mut plc = EipPlc::new(100, path);
    plc.add_tag("BaseBOOL".to_string(), EipTagType::Bool);
    plc.add_tag("BaseREAL".to_string(), EipTagType::Real);

    let value = plc.read_tag("BaseBOOL".to_string());

    match value {
        TagResult::Bool(value) => println!("Bool: {}", value),
        TagResult::Real(value) => println!("Real: {}", value),
        _ => println!("Not matched")
    }

    add_another_tag(&mut plc);

    plc.add_tag("BaseREAL".to_string(), EipTagType::Real);

    //let value2 = plc.read_tag("BaseREAL".to_string());
    //println!("Hello World 2");
    // let value = plc.read_bool("BaseBOOL".to_string());
    // let val_real = plc.read_real("BaseREAL".to_string());
    // println!("{} {}", value, val_real);

    // let val_test = plc.read_test("BaseBOOL".to_string());
    //
    // match val_test {
    //     Err(val_test) => println!("err occurred"),
    //     Ok(val_test) => println!("value exists")
    // }
}

fn add_another_tag(plc: &mut EipPlc) {
    plc.add_tag("BaseDINT".to_string(), EipTagType::Bool);
}