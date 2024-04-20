use enum_desc::{enum_trs, EnumDesc, EnumDescU8};

#[derive(Debug, EnumDesc)]
pub enum GenderEnum {
    #[info(desc = "male")]
    MALE = 0,
    #[info(desc = "female")]
    FEMALE = 1,
}

#[derive(Debug, EnumDescU8)]
pub enum AllowDeviceTypeEnum {
    #[info(desc = "mobile phone")]
    PHONE = 1,

    #[info(desc = "host computer")]
    PC = 2,

    #[info(desc = "tablet")]
    PAD = 4,
}

#[enum_trs(
gender = GenderEnum,
allow_device_type & AllowDeviceTypeEnum,
)]
#[derive(Debug)]
pub struct UserDto {
    pub name: String,
    pub gender: i16,
    pub allow_device_type: Option<u8>,
}

fn main() {
    let mut user_dto = UserDto {
        name: "hui".to_string(),
        gender: 1i16,
        allow_device_type: Some(7u8),
        gender_desc: "".to_string(),
        allow_device_type_desc: "".to_string(),
    };
    user_dto.translate_enums();
    println!("{:#?}", user_dto);
}