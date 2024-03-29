use std::fs::File;
use std::io::{self, BufRead, Read};
use std::panic::panic_any;
use std::panic::{self, catch_unwind, resume_unwind, UnwindSafe};
use std::ptr;
use std::slice;

mod comparer;

pub use self::comparer::try_compare;
pub use super::{error::Error, Status};
pub use unix::UnixFdReader;

fn catch_io<R>(f: impl FnOnce() -> R + UnwindSafe) -> io::Result<R> {
    let hook = panic::take_hook();
    let ret = match catch_unwind(f) {
        Ok(ans) => Ok(ans),
        Err(payload) => match payload.downcast::<io::Error>() {
            Ok(e) => Err(*e),
            Err(payload) => resume_unwind(payload),
        },
    };
    panic::set_hook(hook);
    ret
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoByte {
    byte: u8,
    eof: bool,
}

impl IoByte {
    pub const EOF: IoByte = IoByte { byte: 0, eof: true };

    #[inline(always)]
    pub fn from_u8(byte: u8) -> Self {
        Self { byte, eof: false }
    }

    #[inline(always)]
    pub fn as_u8(self) -> u8 {
        self.byte
    }

    #[inline(always)]
    pub fn is_eof(self) -> bool {
        self.eof
    }
}

pub trait ByteRead: BufRead {
    fn next_byte(&mut self) -> IoByte;

    #[allow(clippy::missing_safety_doc)]
    unsafe fn consume_unchecked(&mut self, amt: usize);
}

impl ByteRead for &'_ [u8] {
    fn next_byte(&mut self) -> IoByte {
        match self {
            [] => IoByte::EOF,
            [byte, remain @ ..] => {
                *self = remain;
                IoByte::from_u8(*byte)
            }
        }
    }
    unsafe fn consume_unchecked(&mut self, amt: usize) {
        *self = &self[amt..];
    }
}

pub unsafe trait TrustedRead: Read {
    #[inline]
    fn trusted_read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let nread = <Self as Read>::read(self, buf)?;
        Ok(nread.min(buf.len()))
    }
}

unsafe impl TrustedRead for File {}

#[derive(Debug)]
pub struct ByteReader<R> {
    inner: R,
    buf: Box<[u8]>,
    head: *const u8,
    tail: *const u8,
}

impl<R: Read> ByteReader<R> {
    pub fn with_capacity(capacity: usize, reader: R) -> Self {
        Self {
            inner: reader,
            buf: vec![0; capacity].into(),
            head: ptr::null(),
            tail: ptr::null(),
        }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn from_raw(buf: *mut [u8], reader: R) -> Self {
        Self {
            inner: reader,
            buf: Box::from_raw(buf),
            head: ptr::null(),
            tail: ptr::null(),
        }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn into_raw(self) -> (*mut [u8], R) {
        let buf = Box::into_raw(self.buf);
        let reader = self.inner;
        (buf, reader)
    }
}

impl<R: Read> Read for ByteReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<R: TrustedRead> BufRead for ByteReader<R> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        if self.head != self.tail {
            let len = self.tail as usize - self.head as usize;
            Ok(unsafe { slice::from_raw_parts(self.head, len) })
        } else {
            let nread = self.inner.trusted_read(&mut *self.buf)?;
            if nread == 0 {
                self.head = ptr::null();
                self.tail = ptr::null();
                Ok(&[])
            } else {
                self.head = self.buf.as_ptr();
                self.tail = unsafe { self.head.add(nread) };
                Ok(unsafe { slice::from_raw_parts(self.head, nread) })
            }
        }
    }

    fn consume(&mut self, amt: usize) {
        self.head = (self.head as usize)
            .saturating_add(amt)
            .min(self.tail as usize) as *const u8;
    }
}

impl<R: TrustedRead> ByteRead for ByteReader<R> {
    #[inline(always)]
    fn next_byte(&mut self) -> IoByte {
        if self.head != self.tail {
            unsafe {
                let byte = *self.head;
                self.head = self.head.add(1);
                IoByte::from_u8(byte)
            }
        } else {
            match self.inner.trusted_read(&mut *self.buf) {
                Ok(nread) => {
                    if nread == 0 {
                        IoByte::EOF
                    } else {
                        unsafe {
                            let byte = *self.buf.as_ptr();
                            self.head = self.buf.as_ptr().add(1);
                            self.tail = self.buf.as_ptr().add(nread);
                            IoByte::from_u8(byte)
                        }
                    }
                }
                Err(e) => panic_any(e),
            }
        }
    }
    unsafe fn consume_unchecked(&mut self, amt: usize) {
        self.head = self.head.add(amt);
    }
}

#[cfg(unix)]
pub mod unix {
    use super::TrustedRead;

    use std::fs::File;
    use std::io::{self, Read};
    use std::os::raw::c_void;
    use std::os::unix::io::AsRawFd;
    use std::panic::panic_any;

    #[derive(Debug)]
    pub struct UnixFdReader {
        file: File,
    }

    impl UnixFdReader {
        pub fn from_file(file: File) -> Self {
            Self { file }
        }
    }

    impl Read for UnixFdReader {
        #[inline(always)]
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            unsafe {
                let buf_ptr: *mut c_void = buf.as_mut_ptr().cast();
                let fd = self.file.as_raw_fd();
                let ret: isize = nix::libc::read(fd, buf_ptr, buf.len());
                if ret < 0 {
                    panic_any(io::Error::last_os_error());
                }
                assert!(ret as usize <= buf.len());
                Ok(ret as usize)
            }
        }
    }

    unsafe impl TrustedRead for UnixFdReader {
        fn trusted_read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            <Self as Read>::read(self, buf)
        }
    }
}
