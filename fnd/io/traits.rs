#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Error
{
    InvalidOpenOptions,
    CannotOpen,
    CannotSeek,
    CannotRead,
    CannotWrite,
    CannotFlush,
    BufferTooLarge,
    UnexpectedEof,
}

pub type Result<T> = core::result::Result<T, Error>;

pub trait Read
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()>
    {
        while !buf.is_empty()
        {
            match self.read(buf)
            {
                Ok(0) => break,
                Ok(n) => buf = &mut buf[n..],
                Err(e) => return Err(e),
            }
        }

        if !buf.is_empty()
        {
            Err(Error::UnexpectedEof)
        }
        else
        {
            Ok(())
        }
    }
}

pub trait Write
{
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, mut buf: &[u8]) -> Result<()>
    {
        while !buf.is_empty()
        {
            match self.write(buf)
            {
                Ok(0) => return Err(Error::CannotWrite),
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}

pub enum SeekOrigin
{
    Start(isize),
    End(isize),
    Current(isize),
}

pub trait Seek
{
    fn seek(&mut self, pos: SeekOrigin) -> Result<usize>;

    fn stream_position(&mut self) -> Result<usize>
    {
        self.seek(SeekOrigin::Current(0))
    }

    fn stream_len(&mut self) -> Result<usize>
    {
        let position = self.stream_position()?;
        let end_pos = self.seek(SeekOrigin::End(0))?;
        self.seek(SeekOrigin::Start(position as isize))?;
        Ok(end_pos)
    }
}
