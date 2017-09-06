use std::mem;
use std::str;
use std::slice;
use std::borrow::Cow;
use std::path::Path;

use memmap::{Mmap, Protection};

use symbolic_common::{Result, ErrorKind};

use types::{CacheFileHeader, StoredSlice, IndexItem};


enum Backing<'a> {
    Buf(Cow<'a, [u8]>),
    Mmap(Mmap),
}

pub struct SymCache<'a> {
    backing: Backing<'a>
}

pub struct Symbol<'a> {
    cache: &'a SymCache<'a>,
}

impl<'a> Backing<'a> {

    fn get_data(&self, start: usize, len: usize) -> Result<&[u8]> {
        let buffer = self.buffer();
        let end = start.wrapping_add(len);
        if end < start || end > buffer.len() {
            Err(ErrorKind::CorruptCacheFile.into())
        } else {
            Ok(&buffer[start..end])
        }
    }

    fn get_slice<T>(&self, offset: usize, count: usize) -> Result<&[T]> {
        let size = mem::size_of::<T>();
        Ok(unsafe {
            slice::from_raw_parts(
                mem::transmute(self.get_data(offset, count * size)?.as_ptr()),
                count
            )
        })
    }

    #[inline(always)]
    fn header(&self) -> Result<&CacheFileHeader> {
        unsafe {
            Ok(mem::transmute(self.get_data(0, mem::size_of::<CacheFileHeader>())?.as_ptr()))
        }
    }

    #[inline(always)]
    fn buffer(&self) -> &[u8] {
        match *self {
            Backing::Buf(ref buf) => buf,
            Backing::Mmap(ref mmap) => unsafe { mmap.as_slice() }
        }
    }
}

fn load_cachefile<'a>(backing: Backing<'a>) -> Result<SymCache<'a>> {
    {
        let header = backing.header()?;
        if header.version != 2 {
            return Err(ErrorKind::UnknownCacheFileVersion(header.version).into());
        }
    }
    Ok(SymCache {
        backing: backing,
    })
}

impl<'a> SymCache<'a> {

    /// Constructs a memdb object from a byte slice cow.
    pub fn from_cow(cow: Cow<'a, [u8]>) -> Result<SymCache<'a>> {
        load_cachefile(Backing::Buf(cow))
    }

    /// Constructs a memdb object from a byte slice.
    pub fn from_slice(buffer: &'a [u8]) -> Result<SymCache<'a>> {
        SymCache::from_cow(Cow::Borrowed(buffer))
    }

    /// Constructs a memdb object from a byte vector.
    pub fn from_vec(buffer: Vec<u8>) -> Result<SymCache<'a>> {
        SymCache::from_cow(Cow::Owned(buffer))
    }

    /// Constructs a memdb object by mmapping a file from the filesystem in.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<SymCache<'a>> {
        let mmap = Mmap::open_path(path, Protection::Read)?;
        load_cachefile(Backing::Mmap(mmap))
    }

    /// All stored slices.
    fn get_stored_slice(&self, idx: u32) -> Result<&StoredSlice> {
        let header = self.backing.header()?;
        let slices: &[StoredSlice] = self.backing.get_slice(
            header.slices_start as usize,
            header.slices_count as usize
        )?;
        Ok(&slices[idx as usize])
    }

    /// Get a string from the file
    fn get_string(&self, idx: u32) -> Result<&str> {
        let slice = self.get_stored_slice(idx)?;
        let bytes = self.backing.get_data(slice.offset as usize,
                                          slice.len as usize)?;
        Ok(str::from_utf8(bytes)?)
    }

    /// The name of the syscache file
    pub fn name(&self) -> Result<&str> {
        let header = self.backing.header()?;
        self.get_string(header.name_id)
    }

    pub fn lookup(&'a self, addr: u64) -> Result<Option<Symbol<'a>>> {
        Ok(None)
    }
}
