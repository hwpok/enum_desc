use enum_desc::{enum_trs, EnumDesc};

#[derive(Debug, EnumDesc)]
pub enum GenderEnum {
    #[info(desc = "male")]
    MALE = 0,
    #[info(desc = "female")]
    FEMALE = 1,
}

#[derive(Debug, EnumDesc)]
pub enum StatusEnum {
    #[info(desc = "normal")]
    NORMAL = 1,

    #[info(desc = "locked")]
    LOCKED = 0,
}

#[enum_trs(
gender = GenderEnum,
status = StatusEnum,
)]
#[derive(Debug)]
pub struct UserDto {
    pub name: String,
    pub gender: i16,
    pub status: Option<i16>,
}

fn main() {
    let mut user_dto = UserDto {
        name: "hui".to_string(),
        gender: 1i16,
        status: Some(1i16),
        gender_desc: "".to_string(),
        status_desc: "".to_string(),
    };
    user_dto.translate_enums();
    println!("{:#?}", user_dto);
}
