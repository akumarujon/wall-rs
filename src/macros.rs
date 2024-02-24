#![allow(unused_imports)]

#[macro_export]
macro_rules! matcher {
    ($token:expr) => {
        match $token {
            Ok(c) => c,
            Err(e) => return Err(e),
        }
    };
}

#[macro_export]
macro_rules! showroom {
    ($token:expr) => {
        match $token {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{:?}", e);
                exit(1)
            }
        }
    };
}

pub(crate) use matcher;
pub(crate) use showroom;
