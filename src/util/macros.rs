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