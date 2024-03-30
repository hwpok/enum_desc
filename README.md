## ENUM_DESC & ENUM_TRS
Two derived macros, EnumDesc and EnumTrs, are provided to add descriptive information to enumerations and translate enumeration codes


## EXAMPLE - EnumDesc
Add descriptive information to enumerations
```rust
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
```
Run the code above, and you will see the following information:  
&nbsp;&nbsp;&nbsp;&nbsp;  enum: Some(PHONE)  
&nbsp;&nbsp;&nbsp;&nbsp;  code: 2  
&nbsp;&nbsp;&nbsp;&nbsp;  desc: "host computer"  
&nbsp;&nbsp;&nbsp;&nbsp;  desc: "tablet"  

## EXAMPLE - enum_trs
Translate enumeration codes
```rust
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
```

Run the code above, and you will see the following information:    
&nbsp;&nbsp;&nbsp;&nbsp;  UserDto {  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;      name: "hui",  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;  gender: 1,  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;  status: Some(  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;  1,  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;  ),  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;  gender_desc: "female",  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;  status_desc: "normal",  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;  }
```


