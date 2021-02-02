pub trait TaskId {}

pub struct Task<T: TaskId> {
    pub id: T,
    pub timestamp: u32,
}

impl<T: TaskId> Task<T> {
    pub fn new(id: T, timestamp: u32) -> Self {
        Self { id, timestamp }
    }
}
