use tokio::sync::oneshot;

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = oneshot::channel();
    (Sender(Some(tx)), Receiver::Waiting(rx))
}

/// Sending side of `Receiver`.
pub struct Sender<T>(Option<oneshot::Sender<T>>);

impl<T> Sender<T> {
    /// Unconditionally send. Panics if sender was already used.
    pub fn send(&mut self, val: T) -> Result<(), T> {
        self.0.take().unwrap().send(val)
    }
}

/// A oneshot::Receiver that holds `T` and has a lazy `.get()` method to wait for and retrieve a reference to the value.
pub enum Receiver<T> {
    Waiting(oneshot::Receiver<T>),
    Received(T),
}

impl<T> Receiver<T> {
    /// Wait for a value to be received, if one has not been already, and return a reference to it.
    pub async fn get(&mut self) -> &T {
        match self {
            Receiver::Waiting(receiver) => {
                let val = receiver.await.unwrap();
                *self = Receiver::Received(val);
                let Receiver::Received(val) = self else { unreachable!() };
                val
            }
            Receiver::Received(val) => val,
        }
    }
}
