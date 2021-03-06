use super::field::GF256;

/// A share used to reconstruct the secret. Can be serialized to and from a byte array.
///
/// Usage example:
/// ```
/// use sharks::{Sharks, Share};
/// # fn send_to_printer(_: Vec<u8>) {}
/// # fn ask_shares() -> Vec<Vec<u8>> {vec![vec![1, 2], vec![2, 3], vec![3, 4]]}
///
/// // Transmit the share bytes to a printer
/// let sharks = Sharks(3);
/// let dealer = sharks.dealer(&[1, 2, 3]);
///
/// // Get 5 shares and print paper keys
/// for s in dealer.take(5) {
///     send_to_printer(Vec::from(&s));
/// };
///
/// // Get share bytes from an external source and recover secret
/// let shares_bytes: Vec<Vec<u8>> = ask_shares();
/// let shares: Vec<Share> = shares_bytes.iter().map(|s| Share::from(s.as_slice())).collect();
/// let secret = sharks.recover(&shares).unwrap();
#[derive(Clone)]
pub struct Share {
    pub x: GF256,
    pub y: Vec<GF256>,
}

/// Obtains a byte vector from a `Share` instance
impl From<&Share> for Vec<u8> {
    fn from(s: &Share) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(s.y.len() + 1);
        bytes.push(s.x.0);
        bytes.extend(s.y.iter().map(|p| p.0));
        bytes
    }
}

/// Obtains a `Share` instance from a byte slice
impl From<&[u8]> for Share {
    fn from(s: &[u8]) -> Share {
        let x = GF256(s[0]);
        let y = s[1..].iter().map(|p| GF256(*p)).collect();
        Share { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::{Share, GF256};

    #[test]
    fn vec_from_share_works() {
        let share = Share {
            x: GF256(1),
            y: vec![GF256(2), GF256(3)],
        };
        let bytes = Vec::from(&share);
        assert_eq!(bytes, vec![1, 2, 3]);
    }

    #[test]
    fn share_from_u8_slice_works() {
        let bytes = [1, 2, 3];
        let share = Share::from(&bytes[..]);
        assert_eq!(share.x, GF256(1));
        assert_eq!(share.y, vec![GF256(2), GF256(3)]);
    }
}
