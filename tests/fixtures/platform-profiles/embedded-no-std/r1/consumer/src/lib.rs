#![no_std]

//! Controlled embedded revision 1 fixture.

pub const FRAME_LEN: usize = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EncodeError {
    BufferTooSmall,
    ReadingOutOfRange,
}

/// Encodes one 12-bit sensor reading into caller-provided storage.
///
/// ```
/// use ferris_profile_embedded_no_std::{FRAME_LEN, encode};
///
/// let mut frame = [0; FRAME_LEN];
/// assert_eq!(encode(7, 0x345, &mut frame), Ok(FRAME_LEN));
/// assert_eq!(frame, [0xa1, 7, 0x03, 0x45]);
/// ```
pub fn encode(sequence: u8, reading: u16, output: &mut [u8]) -> Result<usize, EncodeError> {
    if reading > 0x0fff {
        return Err(EncodeError::ReadingOutOfRange);
    }
    if output.len() < FRAME_LEN {
        return Err(EncodeError::BufferTooSmall);
    }
    output[..FRAME_LEN].copy_from_slice(&[
        0xa1,
        sequence,
        (reading >> 8) as u8,
        reading as u8,
    ]);
    Ok(FRAME_LEN)
}

#[cfg(test)]
mod tests {
    use super::{EncodeError, FRAME_LEN, encode};

    #[test]
    fn encodes_exact_frame() {
        let mut frame = [0; FRAME_LEN];
        assert_eq!(encode(7, 0x345, &mut frame), Ok(FRAME_LEN));
        assert_eq!(frame, [0xa1, 7, 0x03, 0x45]);
    }

    #[test]
    fn rejects_without_mutating_output() {
        let mut short = [0x55; FRAME_LEN - 1];
        assert_eq!(
            encode(1, 1, &mut short),
            Err(EncodeError::BufferTooSmall)
        );
        assert_eq!(short, [0x55; FRAME_LEN - 1]);

        let mut frame = [0x55; FRAME_LEN];
        assert_eq!(
            encode(1, 0x1000, &mut frame),
            Err(EncodeError::ReadingOutOfRange)
        );
        assert_eq!(frame, [0x55; FRAME_LEN]);
    }
}
