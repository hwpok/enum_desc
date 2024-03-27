## ENUM_DESC & ENUM_TRS
```
EnumDesc: This is a derived macro for adding descriptive information to an enumeration
EnumTrs: This attribute macro is used to translate enum fields in data transfer objects.
```

## EXAMPLE - EnumDesc
```
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
    println!("desc: {:#?}", DeviceTypeEnum::got_desc(3));
}

============================================================
will print: 
    enum: Some(
        PHONE,
    )
    code: 2
    desc: "host computer"
    desc: "tablet"
```

## EXAMPLE - enum_trs
```
use macro_lib::{enum_trs, EnumDesc};

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

============================================================
will print: 
UserDto {
    name: "hui",
    gender: 1,
    status: Some(
        1,
    ),
    gender_desc: "female",
    status_desc: "normal",
}

```


