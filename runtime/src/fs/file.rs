use super::*;

pub trait Fnode {
    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<usize>;
}

pub enum FileKind {
    Regular {
        node: Box<dyn Fnode>,
        position: usize,
    },
}

pub struct File {
    kind: FileKind,
}

impl File {
    pub fn from_fnode(fnode: impl Fnode + 'static) -> Self {
        Self {
            kind: FileKind::Regular {
                node: Box::new(fnode),
                position: 0,
            },
        }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        match &mut self.kind {
            FileKind::Regular { node, position } => {
                let len = node.read(*position, buffer)?;
                *position += len;
                Ok(len)
            }
        }
    }
}
