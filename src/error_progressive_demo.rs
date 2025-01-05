#[cfg(test)]
mod test {
    // Use below code for the beginning of the project development
    // Loose: passthrough, str/String
    #[test]
    fn error_for_test() {
        pub type Result<T> = core::result::Result<T, Error>;
        pub type Error = Box<dyn std::error::Error>;
    }

    // Put below code in src/error.rs after the project structure is defined
    // Strict: structured, Enum
    #[test]
    fn error_for_prod() {
        use serde::Serialize;
        use derive_more::From;

        pub type Result<T> = core::result::Result<T, Error>;

        #[derive(Debug, From)]
        pub enum Error {
            #[from]
            Custom(String),

            // Externals
            #[from]
            Io(std::io::Error),
        }

        impl Error {
            pub fn custom(val: impl std::fmt::Display) -> Self {
                Self::Custom(val.to_string())
            }
        }

        impl From<&str> for Error {
            fn from(val: &str) -> Self {
                Self::Custom(val.to_string())
            }
        }

        impl core::fmt::Display for Error {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
                write!(fmt, "{self:?}")
            }
        }

        impl std::error::Error for Error {}
    }
}
