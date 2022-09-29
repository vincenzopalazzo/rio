pub trait Read {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    async fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> {
        todo!()
    }
    async fn read_buf(&mut self, buf: &mut BorrowedCursor<'_>) -> Result<()> {
        todo!()
    }
    async fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        todo!()
    }
    async fn read_buf_exact(&mut self, buf: &mut BorrowedCursor<'_>) -> Result<()> {
        todo!()
    }
    async fn read_buf_vectored(&mut self, bufs: &mut BorrowedSliceCursor<'_>) -> Result<usize> {
        todo!()
    }
    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        todo!()
    }
    async fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
        todo!()
    }

    fn is_read_vectored(&self) -> bool {
        false
    }

    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn bytes(self) -> Bytes<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn chain<R: Read>(self, next: R) -> Chain<Self, R>
    where
        Self: Sized,
    {
        todo!()
    }

    fn take(self, limit: u64) -> Take<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn as_ready(&mut self) -> Option<&mut impl ReadyRead> {
        None
    }

    fn as_owned(&mut self) -> Option<&mut impl OwnedRead> {
        None
    }

    fn as_ready_dyn(&mut self) -> Option<&mut dyn ReadyRead> {
        None
    }

    fn as_owned_dyn(&mut self) -> Option<&mut dyn OwnedRead> {
        None
    }
}
