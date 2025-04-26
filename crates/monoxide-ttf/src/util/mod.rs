use bytes::BufMut;

/// An implementation of [`bytes::BufMut`] that only keeps track of the size of
/// the buffer, not the actual contents.
///
/// With sufficient compiler optimizations, this can be used to calculate the
/// written size of a buffer without actually writing the contents.
///
/// In rare cases where the function [`Self::chunk_mut`] is called, this type
/// will panic, since there is _no_ underlying buffer to write to. Most of the
/// time, this should not be a problem, since users of this type should usually
/// just call high-level ones like [`Self::put_slice`] or [`Self::put_bytes`].
pub struct SizeOnlyBufWriter {
    size: usize,
}

impl SizeOnlyBufWriter {
    pub fn new() -> Self {
        Self { size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Default for SizeOnlyBufWriter {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl BufMut for SizeOnlyBufWriter {
    fn remaining_mut(&self) -> usize {
        usize::MAX
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.size += cnt;
    }

    fn chunk_mut(&mut self) -> &mut bytes::buf::UninitSlice {
        panic!("There's no way to write to this buffer")
    }

    fn put<T: bytes::buf::Buf>(&mut self, src: T)
    where
        Self: Sized,
    {
        self.size += src.remaining();
    }

    fn put_slice(&mut self, src: &[u8]) {
        self.size += src.len();
    }

    fn put_bytes(&mut self, _val: u8, cnt: usize) {
        self.size += cnt;
    }
}
