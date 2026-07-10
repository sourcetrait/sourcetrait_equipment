use crate::generated::ai::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrKind {
    Prefix,
    Char(u8),
    Length { expected: usize, got: usize },
}

pub trait BaseEncDec:
    From<Vec<u8>>
    + AsRef<[u8]>
    + Into<Vec<u8>>
    + for<'a> TryFrom<&'a str, Error = ErrKind>
    + Display
    {
        fn from_bytes<I: Into<Vec<u8>>>(bytes: I) -> Self {
            Self::from(bytes.into())
        }

        fn take(self) -> Vec<u8> { self.into() }

        fn try_into_bytes<const N: usize>(self) -> Result<[u8; N], ErrKind> {
            let bytes: Vec<u8> = self.into();
            let got = bytes.len();
            <[u8; N]>::try_from(bytes).map_err(|_| ErrKind::Length { expected: N, got })
        }
        
        fn as_bytes(&self) -> &[u8] { self.as_ref() }

        fn from_str_static(s: &'static str) -> Self {
            Self::try_from(s).expect("valid")
        }
    }

pub mod base32 {
    pub mod multi {
        pub struct Rfc4648(Vec<u8>);

        mod rfc4648 {
            use super::{Rfc4648, super::super::*};

            const PREFIX: u8 = b'b';                            // multibase code for base32
            const ALPHABET: &[u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";

            impl BaseEncDec for Rfc4648 {}

            impl From<Vec<u8>> for Rfc4648 {
                fn from(value: Vec<u8>) -> Self {
                    Rfc4648(value)
                }
            }

            impl AsRef<[u8]> for Rfc4648 {
                fn as_ref(&self) -> &[u8] {
                    &self.0
                }
            }

            impl Into<Vec<u8>> for Rfc4648 {
                fn into(self) -> Vec<u8> {
                    self.0
                }
            }

            impl TryFrom<&str> for Rfc4648 {
                type Error = ErrKind;

                fn try_from(value: &str) -> Result<Self, Self::Error> {
                    match value.as_bytes().split_first() {
                        Some((&PREFIX, rest)) => Ok(Rfc4648(decode(rest)?)),
                        _ => Err(ErrKind::Prefix),
                    }
                }
            }

            impl Display for Rfc4648 {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}{}", PREFIX as char, encode(&self.0))
                }
            }

            fn encode(bytes: &[u8]) -> String {
                let mut out = String::with_capacity((bytes.len() * 8 + 4) / 5);
                let mut acc: u32 = 0;
                let mut bits: u32 = 0;
                for &b in bytes {
                    acc = (acc << 8) | b as u32;
                    bits += 8;
                    while bits >= 5 {
                        bits -= 5;
                        out.push(ALPHABET[((acc >> bits) & 0x1f) as usize] as char);
                    }
                }
                if bits > 0 {
                    out.push(ALPHABET[((acc << (5 - bits)) & 0x1f) as usize] as char);
                }
                out
            }

            fn decode(s: &[u8]) -> Result<Vec<u8>, ErrKind> {
                let mut out = Vec::with_capacity(s.len() * 5 / 8);
                let mut acc: u32 = 0;
                let mut bits: u32 = 0;
                for &c in s {
                    let val = match c {
                        b'a'..=b'z' => c - b'a',
                        b'2'..=b'7' => c - b'2' + 26,
                        _ => return Err(ErrKind::Char(c)),
                    } as u32;
                    acc = (acc << 5) | val;
                    bits += 5;
                    if bits >= 8 {
                        bits -= 8;
                        out.push((acc >> bits) as u8);
                    }
                }
                Ok(out)
            }
        }
    }
}

pub mod base36 {
    pub struct Multi(Vec<u8>);

    mod multi {
        use super::{Multi, super::*};

        const PREFIX: u8 = b'k';                            // multibase code for base36 (lowercase)
        const ALPHABET: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";

        impl BaseEncDec for Multi {}

        impl From<Vec<u8>> for Multi {
            fn from(value: Vec<u8>) -> Self {
                Multi(value)
            }
        }

        impl AsRef<[u8]> for Multi {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }

        impl Into<Vec<u8>> for Multi {
            fn into(self) -> Vec<u8> {
                self.0
            }
        }

        impl TryFrom<&str> for Multi {
            type Error = ErrKind;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value.as_bytes().split_first() {
                    Some((&PREFIX, rest)) => Ok(Multi(decode(rest)?)),
                    _ => Err(ErrKind::Prefix),
                }
            }
        }

        impl Display for Multi {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}{}", PREFIX as char, encode(&self.0))
            }
        }

        // base36 is not power-of-two: treat the bytes as a big-endian bignum and
        // repeatedly divide by 36. Leading zero bytes map to leading '0' chars.
        fn encode(bytes: &[u8]) -> String {
            let zeros = bytes.iter().take_while(|&&b| b == 0).count();
            let mut digits: Vec<u8> = Vec::new();               // little-endian base36
            for &byte in bytes {
                let mut carry = byte as u32;
                for digit in digits.iter_mut() {
                    carry += (*digit as u32) * 256;
                    *digit = (carry % 36) as u8;
                    carry /= 36;
                }
                while carry > 0 {
                    digits.push((carry % 36) as u8);
                    carry /= 36;
                }
            }
            let mut out = String::with_capacity(zeros + digits.len());
            for _ in 0..zeros {
                out.push(ALPHABET[0] as char);
            }
            for &d in digits.iter().rev() {
                out.push(ALPHABET[d as usize] as char);
            }
            out
        }

        fn decode(s: &[u8]) -> Result<Vec<u8>, ErrKind> {
            let zeros = s.iter().take_while(|&&c| c == b'0').count();
            let mut bytes: Vec<u8> = Vec::new();                // little-endian base256
            for &c in s {
                let val = match c {
                    b'0'..=b'9' => c - b'0',
                    b'a'..=b'z' => c - b'a' + 10,
                    _ => return Err(ErrKind::Char(c)),
                } as u32;
                let mut carry = val;
                for b in bytes.iter_mut() {
                    carry += (*b as u32) * 36;
                    *b = (carry & 0xff) as u8;
                    carry >>= 8;
                }
                while carry > 0 {
                    bytes.push((carry & 0xff) as u8);
                    carry >>= 8;
                }
            }
            let mut out = Vec::with_capacity(zeros + bytes.len());
            for _ in 0..zeros {
                out.push(0u8);
            }
            for &b in bytes.iter().rev() {
                out.push(b);
            }
            Ok(out)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn usage_base32_rfc_4648() {
        use super::{BaseEncDec, base32::multi::Rfc4648 as base32};
        const EXPECTED: &str = "bafkreigh6aess6gkvwadzqlfkxxeyzfofn67g63dkbtruy3ffyl7p65opa";

        let _ = base32::from_str_static(EXPECTED);
        let cid = base32::try_from(EXPECTED).expect("valid");

        assert_eq!(cid.to_string(), EXPECTED);
        assert_eq!(cid.as_bytes().len(), 36);
        assert_eq!(&cid.as_bytes()[..4], &[0x01, 0x55, 0x12, 0x20]);
        assert!(matches!(base32::try_from("b!!!"), Err(super::ErrKind::Char(b'!'))));
        assert!(matches!(base32::try_from("zfoo"), Err(super::ErrKind::Prefix)));
    }

    #[test]
    fn usage_base36_ipns() {
        use super::{BaseEncDec, base36::Multi as base36};
        const EXPECTED: &str = "k51qzi5uqu5dkdbnmhuz1ksriva06ulvmaf3bu8ig8sm7vu3dvzt1gkda0sjee";

        let _ = base36::from_str_static(EXPECTED);
        let ipns = base36::try_from(EXPECTED).expect("valid");

        assert_eq!(ipns.to_string(), EXPECTED);
        assert_eq!(ipns.as_bytes().len(), 40);
        // libp2p-key codec 0x72, identity hash 0x00, 36-byte pubkey digest
        assert_eq!(&ipns.as_bytes()[..4], &[0x01, 0x72, 0x00, 0x24]);
        assert!(matches!(base36::try_from("bfoo"), Err(super::ErrKind::Prefix)));
    }
}
