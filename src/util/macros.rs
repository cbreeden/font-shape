macro_rules! impl_parse {
    ($($trans:ident => $ident:ident; $size:expr),*) => {
        $(
            impl Parse for $ident {
                fn size() -> usize { $size }
                fn parse (buf: &[u8]) -> Result<(&[u8], $ident)> {
                    if buf.len() < Self::size() {
                        return Err(Error::UnexpectedEof)
                    }

                    let res = $ident::from(decode::$trans(buf));
                    Ok((&buf[Self::size()..], res))
                }
            }
        )*
    }
}