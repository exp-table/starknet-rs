mod name_error {
    #[derive(Debug)]
    pub struct NonAsciiNameError;

    #[cfg(feature = "std")]
    impl std::error::Error for NonAsciiNameError {}

    impl core::fmt::Display for NonAsciiNameError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "the provided name contains non-ASCII characters")
        }
    }
}
pub use name_error::NonAsciiNameError;

mod string_error {
    #[derive(Debug)]
    pub enum CairoShortStringToFeltError {
        NonAsciiCharacter,
        StringTooLong,
    }

    #[cfg(feature = "std")]
    impl std::error::Error for CairoShortStringToFeltError {}

    impl core::fmt::Display for CairoShortStringToFeltError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::NonAsciiCharacter => {
                    write!(f, "Cairo string can only contain ASCII characters")
                }
                Self::StringTooLong => {
                    write!(f, "short string exceeds maximum length of 31 characters")
                }
            }
        }
    }
}
pub use string_error::CairoShortStringToFeltError;

mod parsing_error {
    #[derive(Debug)]
    pub enum ParseCairoShortStringError {
        ValueOutOfRange,
        UnexpectedNullTerminator,
    }

    #[cfg(feature = "std")]
    impl std::error::Error for ParseCairoShortStringError {}

    impl core::fmt::Display for ParseCairoShortStringError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::ValueOutOfRange => {
                    write!(f, "field element value out of range")
                }
                Self::UnexpectedNullTerminator => {
                    write!(f, "unexpected null terminator")
                }
            }
        }
    }
}
pub use parsing_error::ParseCairoShortStringError;

mod sign_error {
    #[derive(Debug)]
    pub enum EcdsaSignError {
        MessageHashOutOfRange,
    }

    #[cfg(feature = "std")]
    impl std::error::Error for EcdsaSignError {}

    impl core::fmt::Display for EcdsaSignError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::MessageHashOutOfRange => {
                    write!(f, "message hash out of range")
                }
            }
        }
    }
}
pub use sign_error::EcdsaSignError;

mod verify_error {
    #[derive(Debug)]
    pub enum EcdsaVerifyError {
        MessageHashOutOfRange,
        InvalidPublicKey,
        SignatureROutOfRange,
        SignatureSOutOfRange,
    }

    #[cfg(feature = "std")]
    impl std::error::Error for EcdsaVerifyError {}

    impl core::fmt::Display for EcdsaVerifyError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::MessageHashOutOfRange => {
                    write!(f, "message hash out of range")
                }
                Self::InvalidPublicKey => {
                    write!(f, "invalid public key")
                }
                Self::SignatureROutOfRange => {
                    write!(f, "signature r value out of range")
                }
                Self::SignatureSOutOfRange => {
                    write!(f, "signature s value out of range")
                }
            }
        }
    }
}
pub use verify_error::EcdsaVerifyError;
