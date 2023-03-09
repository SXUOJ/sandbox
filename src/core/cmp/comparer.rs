use super::{catch_io, ByteRead, Error, IoByte, Status};
use std::cmp::{Ord, Ordering};
use std::panic::{panic_any, AssertUnwindSafe};

pub fn try_compare(
    std_reader: &mut impl ByteRead,
    user_reader: &mut impl ByteRead,
) -> Result<Status, Error> {
    catch_io(AssertUnwindSafe(move || compare(std_reader, user_reader)))
        .map_err(Error::CompareError)
}

#[inline(never)]
fn compare(std_reader: &mut impl ByteRead, user_reader: &mut impl ByteRead) -> Status {
    let mut std_byte = std_reader.next_byte();
    let mut user_byte = user_reader.next_byte();

    let mut ans = Status::Accepted;

    loop {
        if std_byte.is_eof() {
            return handle_eof(user_reader, user_byte, ans);
        }

        if user_byte.is_eof() {
            return handle_eof(std_reader, std_byte, ans);
        }

        let (a, b) = (std_byte.as_u8(), user_byte.as_u8());
        if a == b {
            let ret = poll_diff(std_reader, user_reader);
            std_byte = ret.0;
            user_byte = ret.1;
            continue;
        }

        if a == b'\n' {
            if !b.is_ascii_whitespace() {
                return Status::WrongAnswer;
            }
            if poll_endline(user_reader) {
                std_byte = std_reader.next_byte();
                user_byte = user_reader.next_byte();
                continue;
            } else {
                return Status::WrongAnswer;
            }
        }
        if b == b'\n' {
            if !a.is_ascii_whitespace() {
                return Status::WrongAnswer;
            }
            if poll_endline(std_reader) {
                std_byte = std_reader.next_byte();
                user_byte = user_reader.next_byte();
                continue;
            } else {
                return Status::WrongAnswer;
            }
        }

        let flaga = a.is_ascii_whitespace();
        let flagb = b.is_ascii_whitespace();

        // a != b
        // both of them are non-space
        if !flaga & !flagb {
            return Status::WrongAnswer;
        }

        // a != b
        // both of them are not non-space
        if flaga {
            std_byte = poll_nonspace(std_reader);
        }
        if flagb {
            user_byte = poll_nonspace(user_reader);
        }

        if std_byte.is_eof() || user_byte.is_eof() {
            continue;
        }

        let (a, b) = (std_byte.as_u8(), user_byte.as_u8());
        let flaga = a == b'\n';
        let flagb = b == b'\n';

        if flaga & flagb {
            std_byte = std_reader.next_byte();
            user_byte = user_reader.next_byte();
            continue;
        }
        if flaga | flagb {
            return Status::WrongAnswer;
        }
        if a == b {
            ans = Status::PresentationError;
            std_byte = std_reader.next_byte();
            user_byte = user_reader.next_byte();
            continue;
        } else {
            return Status::WrongAnswer;
        }
    }
}

#[inline(never)]
fn handle_eof(rhs: &mut impl ByteRead, rhs_byte: IoByte, ans: Status) -> Status {
    if rhs_byte.is_eof() {
        return ans;
    }
    if !rhs_byte.as_u8().is_ascii_whitespace() {
        return Status::WrongAnswer;
    }
    if poll_eof(rhs) {
        ans
    } else {
        Status::WrongAnswer
    }
}

#[inline]
fn poll_diff(lhs: &mut impl ByteRead, rhs: &mut impl ByteRead) -> (IoByte, IoByte) {
    {
        let lhs_buf = match lhs.fill_buf() {
            Ok(b) => b,
            Err(e) => panic_any(e),
        };
        let rhs_buf = match rhs.fill_buf() {
            Ok(b) => b,
            Err(e) => panic_any(e),
        };
        if lhs_buf.len() >= 8 && rhs_buf.len() >= 8 {
            unsafe {
                let lhs_buf = lhs_buf.get_unchecked(..8);
                let rhs_buf = rhs_buf.get_unchecked(..8);
                if lhs_buf == rhs_buf {
                    lhs.consume_unchecked(8);
                    rhs.consume_unchecked(8);
                }
            }
        }
    }

    let mut lhs_byte;
    let mut rhs_byte;
    let mut eq_cnt: usize = 0;
    let mut cmp_cnt: usize = 0;

    loop {
        lhs_byte = lhs.next_byte();
        rhs_byte = rhs.next_byte();
        cmp_cnt += 1;

        if cmp_cnt >= 1024 {
            break;
        }

        if lhs_byte == rhs_byte {
            eq_cnt += 1;
            if lhs_byte.is_eof() {
                return (lhs_byte, rhs_byte);
            }
        } else {
            return (lhs_byte, rhs_byte);
        }
    }

    loop {
        if cmp_cnt >= 1024 && eq_cnt > cmp_cnt * 255 / 256 {
            let len = diff_block(lhs, rhs);
            if len == 0 {
                eq_cnt = 0;
                cmp_cnt = 0;
            } else {
                eq_cnt += len;
                cmp_cnt += len;
            }
        }

        lhs_byte = lhs.next_byte();
        rhs_byte = rhs.next_byte();
        cmp_cnt += 1;

        if lhs_byte == rhs_byte {
            eq_cnt += 1;
            if lhs_byte.is_eof() {
                return (lhs_byte, rhs_byte);
            }
        } else {
            return (lhs_byte, rhs_byte);
        }
    }
}

#[inline]
fn diff_block(lhs: &mut impl ByteRead, rhs: &mut impl ByteRead) -> usize {
    let mut total: usize = 0;
    loop {
        let lhs_buf: &[u8] = match lhs.fill_buf() {
            Ok(b) => b,
            Err(e) => panic_any(e),
        };

        let rhs_buf: &[u8] = match rhs.fill_buf() {
            Ok(b) => b,
            Err(e) => panic_any(e),
        };

        let (lhs_buf, rhs_buf, len) = match lhs_buf.len().cmp(&rhs_buf.len()) {
            Ordering::Equal => (lhs_buf, rhs_buf, lhs_buf.len()),
            Ordering::Less => (lhs_buf, &rhs_buf[..lhs_buf.len()], lhs_buf.len()),
            Ordering::Greater => (&lhs_buf[..rhs_buf.len()], rhs_buf, rhs_buf.len()),
        };

        if len == 0 {
            break total;
        }

        if lhs_buf == rhs_buf {
            lhs.consume(len);
            rhs.consume(len);
            total += len;
            continue;
        } else {
            break 0;
        }
    }
}

/// poll until eof.
/// ensure that all chars remaining in `chars` are ascii whitespaces
#[inline]
fn poll_eof(reader: &mut impl ByteRead) -> bool {
    loop {
        let b = reader.next_byte();
        if b.is_eof() {
            return true;
        }
        if !b.as_u8().is_ascii_whitespace() {
            return false;
        }
    }
}

/// poll until b'\n'.
/// ensure that all chars remaining in `chars` line are ascii whitespaces
#[inline(always)]
fn poll_endline(reader: &mut impl ByteRead) -> bool {
    let mut b = reader.next_byte();
    loop {
        if b.is_eof() || b.as_u8() == b'\n' {
            return true;
        }
        if !b.as_u8().is_ascii_whitespace() {
            return false;
        }
        b = reader.next_byte();
    }
}

/// poll until b'\n' or non-space or EOF
#[inline(always)]
fn poll_nonspace(reader: &mut impl ByteRead) -> IoByte {
    loop {
        let b: IoByte = reader.next_byte();
        if b.is_eof() || b.as_u8() == b'\n' || !b.as_u8().is_ascii_whitespace() {
            return b;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! judge {
        ($ret:expr, $std:expr,$user:expr) => {{
            let mut std: &[u8] = $std.as_ref();
            let mut user: &[u8] = $user.as_ref();

            let ret = compare(&mut std, &mut user);
            assert_eq!(ret, $ret);
        }};
    }

    #[test]
    fn test_comparer() {
        judge!(Status::Accepted, b"1", b"1");
        judge!(Status::WrongAnswer, b"1", b"2");
        judge!(Status::PresentationError, b"1\n 2", b"1\n2");
    }
}
