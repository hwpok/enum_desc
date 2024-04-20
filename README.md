## ENUM_DESC & ENUM_TRS
A set of column-derived macros is provided to add descriptive information to enumerations and facilitate the translation of enumeration codes.   
Specifically, the following macros serve to augment enums with additional functionality:
"EnumDesc", "EnumDescI8", "EnumDescU8", "EnumDescI16", "EnumDescU16", "EnumDescI32", "EnumDescU32", "EnumDescI64", "EnumDescU64", "EnumDescISize", "EnumDescUSize"  
The enum_trs attribute macro, on the other hand, enables the translation of values, supporting both "equivalent translation" and "flag translation."


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
/// Ordinary Enumeration
#[derive(Debug, EnumDesc)]
pub enum GenderEnum {
    #[info(desc = "male")]
    MALE = 0,
    #[info(desc = "female")]
    FEMALE = 1,
}

/// Flagged Bit Enumeration, 
#[derive(Debug, EnumDescU8)]
pub enum AllowDeviceTypeEnum {
    #[info(desc = "mobile phone")]
    PHONE = 1,

    #[info(desc = "host computer")]
    PC = 2,

    #[info(desc = "tablet")]
    PAD = 4,
}

/// "=" is used for equivalent translation
/// "&"  is used for flag bit translation.
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
```

Run the code above, and you will see the following information:    
&nbsp;&nbsp;&nbsp;&nbsp;  UserDto {  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;      name: "hui",  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;  gender: 1,  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;  allow_device_type: Some(  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;  7,  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;  ),  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;  gender_desc: "female",  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; allow_device_type_desc: "mobile phone | host computer | tablet",  
&nbsp;&nbsp;&nbsp;&nbsp; }
```


