pub trait StatusEnum: Sized {
    fn from_string(s: &str) -> Self;
}

#[macro_export]
macro_rules! impl_status_enum {
    ($name:ident, { $($variant:ident => $pattern:literal),+ $(,)? }) => {
        impl $crate::StatusEnum for $name {
            fn from_string(s: &str) -> Self {
                match s.to_lowercase().as_str() {
                    $($pattern => Self::$variant,)+
                    _ => Self::Unknown,
                }
            }
        }
    };
}
