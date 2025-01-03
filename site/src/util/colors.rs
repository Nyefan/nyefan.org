use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) struct Color {
    #[cfg_attr(not(test), expect(dead_code))]
    value: u32,
    hex: &'static str,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hex)
    }
}

/// A helper `const fn` to convert a color value to an array of bytes (for `"#RRGGBB"` format).
const fn to_hex_bytes(value: u32) -> [u8; 7] {
    const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";
    [
        b'#', // Prefix
        HEX_CHARS[((value >> 20) & 0xF) as usize],
        HEX_CHARS[((value >> 16) & 0xF) as usize],
        HEX_CHARS[((value >> 12) & 0xF) as usize],
        HEX_CHARS[((value >> 8) & 0xF) as usize],
        HEX_CHARS[((value >> 4) & 0xF) as usize],
        HEX_CHARS[((value >> 0) & 0xF) as usize],
    ]
}

/// Macro to generate a compile-time static `Color`.
macro_rules! color {
    ($name:ident, $value:literal) => {
        pub(crate) const $name: Color = Color {
            value: $value,
            hex: {
                // SAFETY: this is evaluated for every color at compile time, and the values of all
                // 7 u8s are constrained by HEX_CHARS in to_hex_bytes, and the idempotency test is
                // automatically generated below using non-const stdlib functions
                const HEX_STR: &str = unsafe {
                    const HEX_ARRAY: [u8; 7] = to_hex_bytes($value);
                    std::str::from_utf8_unchecked(&HEX_ARRAY)
                };
                HEX_STR
            },
        };

        #[cfg(test)]
        #[allow(non_snake_case)]
        #[test_log::test]
        fn ${concat(test_idempotency_, $name)}() {
            assert_eq!(
                format!("#{:06x}", $name.value).to_uppercase(),
                $name.hex,
                "hex value for {} does not match expected value!",
                stringify!($name)
            );
            assert_eq!(
                u32::from_str_radix($name.hex.trim_start_matches("#"), 16).unwrap(),
                $name.value,
                "u32 value for {} does not match expected value!",
                stringify!($name)
            );
            tracing::info!("{:?}", $name);
        }
    };
}

// Define colors as constants
color!(BEIGE_LIGHT, 0xEEEDE7);
color!(GOLDENROD_DARK, 0xD1A74F);
color!(GRAY_DARCULA, 0x2B2B2B);
color!(LAVENDER_DARK, 0x4E2A84);
color!(LAVENDER_MEDIUM, 0x7851A9);
color!(JADE, 0x256B45);
color!(WHITE_SOFT, 0xFAFAFA);
