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

macro_rules! _offset {
    (NUL) => { 0 };

    (u8)  => { 1 };
    (u16) => { 2 };
    (u32) => { 4 };
    (u64) => { 8 };

    (i8)  => { 1 };
    (i16) => { 2 };
    (i32) => { 4 };
    (i64) => { 8 };

    (Panose) => { 10 };
    (Tag)    => { 4 };
}

macro_rules! offsets {
    (@item $pname:tt $pty:tt $name:tt: $ty:tt,) => {
        pub const $name: usize = $pname + _offset!($pty);
    };

    (@item $pname:tt $pty:tt $name:tt: $ty:tt, $($tail:tt)*) => {
        pub const $name: usize = $pname + _offset!($pty);
        offsets!(@item $name $ty $($tail)*);
    };

    (@group $name:tt $($tail:tt)*) => {
        pub mod $name {
            offsets!(@item 0 NUL $($tail)*);
        }
    };

    // Match for multiple namespaces
    ($($name:ident { $($body:tt)* },)* $(,)*) => {
        mod offsets {
            #![allow(non_upper_case_globals)]
            #![allow(dead_code)]
            $( offsets!(@group $name $($body)*); )*
        }
    };

    // Otherwise match for a single namespace
    ($($tail:tt)*) => {
        mod offsets {
            #![allow(non_upper_case_globals)]
            #![allow(dead_code)]
            offsets!(@item 0 NUL $($tail)*);
        }
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

        let file = File::open($name).expect("unable to open file");

        let mut reader = BufReader::new(file);
        let mut data = Vec::new();
        reader.read_to_end(&mut data).expect("error reading font");

        data
    })
}