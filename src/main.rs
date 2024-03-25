use enum_desc::EnumDesc;

#[derive(EnumDesc, Debug)]
pub enum DeviceTypeEnum {
    #[info(desc = "mobile phone")]
    PHONE = 1,

    #[info(desc = "host computer")]
    PC = 2,

    #[info(desc = "tablet")]
    PAD = 3,
}

fn main() {
    println!("enum: {:#?}", DeviceTypeEnum::from_code(1));
    println!("code: {:#?}", DeviceTypeEnum::PC.to_code());
    println!("desc: {:#?}", DeviceTypeEnum::PC.get_desc());
    println!("desc2: {:#?}", DeviceTypeEnum::get_desc2(3));
}