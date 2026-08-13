#![no_std]

//! Controlled embedded revision 2 fixture.

pub const FRAME_LEN: usize = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EncodeError {
    BufferTooSmall,
    InvalidFlags,
    ReadingOutOfRange,
}

/// Encodes one 12-bit sensor reading, status flags, and checksum.
///
/// ```
/// use ferris_profile_embedded_no_std::{FRAME_LEN, encode};
///
/// let mut frame = [0; FRAME_LEN];
/// assert_eq!(encode(7, 0x03, 0x345, &mut frame), Ok(FRAME_LEN));
/// assert_eq!(frame, [0xa2, 7, 0x03, 0x03, 0x45, 0xe0]);
/// ```
pub fn encode(
    sequence: u8,
    flags: u8,
    reading: u16,
    output: &mut [u8],
) -> Result<usize, EncodeError> {
    if flags > 0x0f {
        return Err(EncodeError::InvalidFlags);
    }
    if reading > 0x0fff {
        return Err(EncodeError::ReadingOutOfRange);
    }
    if output.len() < FRAME_LEN {
        return Err(EncodeError::BufferTooSmall);
    }
    let mut frame = [
        0xa2,
        sequence,
        flags,
        (reading >> 8) as u8,
        reading as u8,
        0,
    ];
    frame[FRAME_LEN - 1] = frame[..FRAME_LEN - 1]
        .iter()
        .copied()
        .fold(0, core::ops::BitXor::bitxor);
    output[..FRAME_LEN].copy_from_slice(&frame);
    Ok(FRAME_LEN)
}

#[cfg(test)]
mod tests {
    use super::{EncodeError, FRAME_LEN, encode};

    #[test]
    fn encodes_exact_frame_with_checksum() {
        let mut frame = [0; FRAME_LEN];
        assert_eq!(encode(7, 0x03, 0x345, &mut frame), Ok(FRAME_LEN));
        assert_eq!(frame, [0xa2, 7, 0x03, 0x03, 0x45, 0xe0]);
    }

    #[test]
    fn rejects_without_mutating_output() {
        let mut short = [0x55; FRAME_LEN - 1];
        assert_eq!(
            encode(1, 0, 1, &mut short),
            Err(EncodeError::BufferTooSmall)
        );
        assert_eq!(short, [0x55; FRAME_LEN - 1]);

        let mut frame = [0x55; FRAME_LEN];
        assert_eq!(
            encode(1, 0x10, 1, &mut frame),
            Err(EncodeError::InvalidFlags)
        );
        assert_eq!(frame, [0x55; FRAME_LEN]);
        assert_eq!(
            encode(1, 0, 0x1000, &mut frame),
            Err(EncodeError::ReadingOutOfRange)
        );
        assert_eq!(frame, [0x55; FRAME_LEN]);
    }
}
