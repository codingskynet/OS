use tar_no_std::{ArchiveEntry, TarArchiveRef};

use super::*;
use crate::fs::file::Fnode;

pub struct Tarfs {
    entries: HashMap<Path, ArchiveEntry<'static>>,
}

impl From<&'static [u8]> for Tarfs {
    fn from(value: &'static [u8]) -> Self {
        Self {
            entries: TarArchiveRef::new(value)
                .unwrap()
                .entries()
                .map(|entry| {
                    (
                        Path::from_str(entry.filename().as_str().unwrap()).unwrap(),
                        entry,
                    )
                })
                .collect(),
        }
    }
}

impl Fs for Tarfs {
    fn open(&self, path: &Path) -> Result<File> {
        match self.entries.get(path) {
            Some(entry) => Ok(File::from_fnode(TarfsFnode::new(entry.data()))),
            None => Err(Error::NotFound),
        }
    }
}

pub struct TarfsFnode<'a> {
    data: &'a [u8],
}

impl<'a> TarfsFnode<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
}

impl<'a> Fnode for TarfsFnode<'a> {
    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<usize> {
        let Some(data) = self.data.get(offset..) else {
            return Ok(0);
        };

        let len = data.len().min(buffer.len());
        buffer[..len].copy_from_slice(&data[..len]);
        Ok(len)
    }
}
