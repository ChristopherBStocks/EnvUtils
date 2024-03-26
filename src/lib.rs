// Re-exports
pub use logic::defaults::*;
pub mod libraries;

mod logic {
    pub mod defaults;
}

#[cfg(feature = "testing")]
#[cfg(test)]
mod tests {
    mod logic {
        mod defaults_test;
    }
}