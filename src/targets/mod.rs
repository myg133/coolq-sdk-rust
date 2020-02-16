use std::io::{Cursor, Error, Read, Result as IOResult};
use std::os::raw::c_char;

use byteorder::{BigEndian, ReadBytesExt};

#[macro_use]
use crate::utf8;
use crate::api::{set_group_anonymous_ban, Flag};

pub mod cqcode;
pub mod message;

pub mod group;
pub mod user;

pub(crate) fn read_multi_object(b: &[u8]) -> IOResult<Vec<Vec<u8>>> {
    let mut b = Cursor::new(base64::decode(&b).unwrap());
    let count = b.read_i32::<BigEndian>()?;
    let mut vs = Vec::new();
    for _ in 0..count {
        let mut v = vec![0u8; b.read_i16::<BigEndian>()? as usize];
        b.read_exact(&mut v)?;
        vs.push(v);
    }
    Ok(vs)
}

pub trait ReadString: Read {
    fn read_string(&mut self) -> IOResult<String> {
        let len = self.read_i16::<BigEndian>()?;
        if len > 0 {
            let mut v = vec![0u8; len as usize];
            self.read_exact(&mut v)?;
            Ok(utf8!(v.as_slice().as_ptr() as *const c_char))
        } else {
            Ok(String::new())
        }
    }
}

impl<R: Read + ?Sized> ReadString for R {}

#[derive(Debug, Clone)]
pub struct File {
    pub id: String,
    pub name: String,
    pub size: i64,
    pub busid: i64,
}

impl File {
    pub(crate) fn decode(b: &[u8]) -> IOResult<File> {
        let mut b = Cursor::new(base64::decode(&b).unwrap());
        Ok(File {
            id: b.read_string()?,
            name: b.read_string()?,
            size: b.read_i64::<BigEndian>()?,
            busid: b.read_i64::<BigEndian>()?,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct Anonymous {
    pub group_id: i64,
    pub user_id: i64,
    pub name: String,
    pub flag: Flag,
}

impl Anonymous {
    pub fn ban(&self, time: i64) {
        set_group_anonymous_ban(self.group_id, self.flag.clone(), time);
    }

    pub(crate) fn decode(b: &[u8], group_id: i64) -> IOResult<Anonymous> {
        let mut c = Cursor::new(base64::decode(&b).unwrap());
        Ok(Anonymous {
            group_id: group_id,
            user_id: c.read_i64::<BigEndian>()?,
            name: c.read_string()?,
            flag: unsafe { String::from_utf8_unchecked(b.to_vec()) },
        })
    }
}
