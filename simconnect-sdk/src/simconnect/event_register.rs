use crate::SimConnectError;

#[derive(Debug)]
pub(crate) struct EventRegister<T>
where
    T: std::fmt::Debug + std::cmp::PartialEq,
{
    items: Vec<T>,
}

impl<T> EventRegister<T>
where
    T: std::fmt::Debug + std::cmp::PartialEq,
{
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn is_registered(&self, item: T) -> bool {
        self.items.contains(&item)
    }

    pub fn register(&mut self, item: T) -> Result<(), SimConnectError> {
        if self.items.contains(&item) {
            return Err(SimConnectError::EventAlreadySubscribedTo(format!(
                "{item:?}"
            )));
        }

        self.items.push(item);

        Ok(())
    }

    pub fn unregister(&mut self, item: T) -> Result<(), SimConnectError> {
        if !self.items.contains(&item) {
            return Err(SimConnectError::EventNotSubscribedTo(format!("{item:?}")));
        }

        self.items.retain(|i| *i != item);

        Ok(())
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}
