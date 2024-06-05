use crate::id::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Peer {
    id: Id,
}

impl Peer {
    pub fn new(id: Id) -> Self {
        Self { id }
    }

    pub fn distance(&self, other: &Id) -> Id {
        self.id.distance(other)
    }
}