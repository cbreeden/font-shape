macro_rules! impl_parse {
    ($($trans:ident => $ident:ident; $size:expr),*) => {
        $(
            impl StaticSize for $ident {
                fn static_size() -> usize { $size }
            }

            impl Table for $ident {
                fn parse (buf: &[u8]) -> Result<(&[u8], $ident)> {
                    if buf.len() < Self::static_size() {
                        return Err(Error::UnexpectedEof)
                    }

                    let res = $ident::from(decode::$trans(buf));
                    Ok((&buf[Self::static_size()..], res))
                }
            }
        )*
    }
}

macro_rules! static_size {
    ($($name:ident = $size:expr),*) => {
        $(
            impl StaticSize for $name {
                fn static_size() -> usize { $size }
            }
        )*
    }
}

macro_rules! versioned_table {
    ($name:ty, $i:ty => |$arg:pat| $body:block) => {
        impl Table for $name {
            fn parse(buf: &[u8]) -> Result<(&[u8], $name)> {
                if buf.len() < Self::static_size() {
                    return Err(Error::UnexpectedEof)
                }

                let (buf, $arg) = <$i>::parse(buf)?;
                let res = $body;

                Ok((buf, res))
            }
        }
    }
}

// versioned_table!(Version,
//     Tag => |tag| {
//         const VERSION1: &[u8; 4] = &[0x00, 0x01, 0x00, 0x00];
//         match &tag.0 {
//             b"OTTO" => Version::OpenType,
//             VERSION1 | b"true" | b"typ1" => Version::TrueType,
//             b"ttcf" => return Err(Error::TtcfUnsupported),
//             _ => return Err(Error::InvalidData),
//         }
//     }
// );

// impl Table for Version {
//     fn parse(buf: &[u8]) -> Result<(&[u8], Version)> {
//         const VERSION1: &[u8; 4] = &[0x00, 0x01, 0x00, 0x00];

//         if buf.len() < Self::static_size() {
//             return Err(Error::UnexpectedEof)
//         }

//         let (buf, tag) = Tag::parse(buf)?;
//         let ver = match &tag.0 {
//             b"OTTO" => Version::OpenType,
//             VERSION1 | b"true" | b"typ1" => Version::TrueType,
//             b"ttcf" => return Err(Error::TtcfUnsupported),
//             _ => return Err(Error::InvalidData),
//         };

//         Ok((buf, ver))
//     }
// }