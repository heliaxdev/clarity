use crate::Error;

use std::str;

/// A function that takes a hexadecimal representation of bytes
/// back into a stream of bytes.
pub fn hex_str_to_bytes(s: &str) -> Result<Vec<u8>, Error> {
    let s = match s.strip_prefix("0x") {
        Some(s) => s,
        None => s,
    };
    let bytes = s
        .as_bytes()
        .chunks(2)
        .map::<Result<u8, Error>, _>(|ch| {
            let str = str::from_utf8(ch)?;
            let byte = u8::from_str_radix(str, 16)?;

            Ok(byte)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(bytes)
}

pub fn bytes_to_hex_str(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:0>2x?}", b))
        .fold(String::new(), |acc, x| acc + &x)
}

#[cfg(test)]
mod test_utils {
    use super::*;

    #[test]
    fn decode_bytes() {
        assert_eq!(
            hex_str_to_bytes("deadbeef").expect("Unable to decode"),
            [222, 173, 190, 239]
        );
    }

    #[test]
    fn decode_odd_amount_of_bytes() {
        assert_eq!(hex_str_to_bytes("f").unwrap(), vec![15]);
    }

    #[test]
    fn bytes_raises_decode_error() {
        let e = hex_str_to_bytes("\u{012345}deadbeef").unwrap_err();

        match e {
            Error::InvalidUtf8(_) => {}
            _ => panic!(),
        };
    }

    #[test]
    fn bytes_raises_parse_error() {
        let e = hex_str_to_bytes("Lorem ipsum").unwrap_err();
        match e {
            Error::InvalidHex(_) => {}
            _ => panic!(),
        }
    }

    #[test]
    fn parse_prefixed_empty() {
        assert_eq!(hex_str_to_bytes("0x").unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn parse_prefixed_non_empty() {
        assert_eq!(
            hex_str_to_bytes("0xdeadbeef").unwrap(),
            vec![0xde, 0xad, 0xbe, 0xef]
        );
    }

    #[test]
    fn encode_bytes() {
        assert_eq!(bytes_to_hex_str(&[0xf]), "0f".to_owned());
        assert_eq!(bytes_to_hex_str(&[0xff]), "ff".to_owned());
        assert_eq!(
            bytes_to_hex_str(&[0xde, 0xad, 0xbe, 0xef]),
            "deadbeef".to_owned()
        );
    }
}
