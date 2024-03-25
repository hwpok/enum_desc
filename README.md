## ENUM_DESC
```
This is a derived macro for adding descriptive information to an enumeration
```

## EXAMPLE
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
```

## INSTRUCTION
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

The EnumDesc derived macro will automatically implement the following functions for DeviceTypeEnum:
impl DeviceTypeEnum {
    pub fn to_code(&self) -> i16 {
        match self {
            Self::PHONE => 1,
            Self::PC => 2,
            Self::PAD => 3,
        }
    }
    #[inline]
    pub fn get_desc(&self) -> &'static str {
        match self {
            Self::PHONE => "mobile phone",
            Self::PC => "host computer",
            Self::PAD => "tablet",
        }
    }
    pub fn from_code(code: i16) -> Option<Self> {
        match code {
            1 => Some(Self::PHONE),
            2 => Some(Self::PC),
            3 => Some(Self::PAD),
            _ => None,
        }
    }
    pub fn got_desc(code: i16) -> &'static str {
        match Self::from_code(code) {
            Some(Self::PHONE) => "mobile phone",
            Some(Self::PC) => "host computer",
            Some(Self::PAD) => "tablet",
            None => "",
        }
    }
}


```


