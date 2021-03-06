use StreamId;
use frame::{self, Head, Kind, Error};

use bytes::{BufMut, BigEndian};

const SIZE_INCREMENT_MASK: u32 = 1 << 31;

#[derive(Copy, Clone, Debug)]
pub struct WindowUpdate {
    stream_id: StreamId,
    size_increment: u32,
}

impl WindowUpdate {
    pub fn new(stream_id: StreamId, size_increment: u32) -> WindowUpdate {
        WindowUpdate {
            stream_id,
            size_increment,
        }
    }

    pub fn stream_id(&self) -> StreamId {
        self.stream_id
    }

    pub fn size_increment(&self) -> u32 {
        self.size_increment
    }

    /// Builds a `WindowUpdate` frame from a raw frame.
    pub fn load(head: Head, payload: &[u8]) -> Result<WindowUpdate, Error> {
        debug_assert_eq!(head.kind(), ::frame::Kind::WindowUpdate);
        if payload.len() != 4 {
            return Err(Error::BadFrameSize);
        }

        // Clear the most significant bit, as that is reserved and MUST be ignored
        // when received.
        let size_increment = unpack_octets_4!(payload, 0, u32) & !SIZE_INCREMENT_MASK;

        // TODO: the size_increment must be greater than 0

        Ok(WindowUpdate {
            stream_id: head.stream_id(),
            size_increment,
        })
    }

    pub fn encode<B: BufMut>(&self, dst: &mut B) {
        trace!("encoding WINDOW_UPDATE; id={:?}", self.stream_id);
        let head = Head::new(Kind::WindowUpdate, 0, self.stream_id);
        head.encode(4, dst);
        dst.put_u32::<BigEndian>(self.size_increment);
    }
}

impl<B> From<WindowUpdate> for frame::Frame<B> {
    fn from(src: WindowUpdate) -> Self {
        frame::Frame::WindowUpdate(src)
    }
}
