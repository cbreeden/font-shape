#[allow(unused_macros)]
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

#[allow(unused_macros)]
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

#[allow(unused_macros)]
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