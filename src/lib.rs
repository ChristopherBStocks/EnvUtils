// Re-exports
pub use logic::defaults::*;
pub mod libraries;

mod logic {
    pub mod defaults {
        mod string;
        pub use string::*;
        mod signed_integer;
        pub use signed_integer::*;
        mod unsigned_integer;
        pub use unsigned_integer::*;
        mod floating_point;
        pub use floating_point::*;
        mod boolean;
        pub use boolean::*;
    }
}

#[cfg(feature = "testing")]
#[cfg(test)]
mod tests {
    mod logic {
        mod defaults {
            mod boolean;
            mod floating_point;
            mod signed_integer;
            mod string;
            mod unsigned_integer;
        }
    }
}