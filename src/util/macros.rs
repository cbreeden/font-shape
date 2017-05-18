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
                Ok((buf, $body))
            }
        }
    };

    ($name:ty, $i:ty => |$buf: pat, $arg:pat| $body:block) => {
        impl Table for $name {
            fn parse(buf: &[u8]) -> Result<(&[u8], $name)> {
                if buf.len() < Self::static_size() {
                    return Err(Error::UnexpectedEof)
                }

                let ($buf, $arg) = <$i>::parse(buf)?;
                Ok($body)
            }
        }
    }
}

macro_rules! tag {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        ::decode::primitives::Tag([$a as u8, $b as u8, $c as u8, $d as u8])
    }
}

//
// Test related Macros
//

macro_rules! open_font {
    ($name:expr) => ({
        use std::fs::File;
        use std::io::BufReader;
        use std::io::prelude::*;

        let file = File::open(r"data/OpenSans-Regular.ttf").expect("unable to open file");

        let mut reader = BufReader::new(file);
        let mut data = Vec::new();
        reader.read_to_end(&mut data).expect("error reading font");

        Font::from_buffer(&data).expect("unable to parse font")        
    })
}