use std::pin::Pin;
use std::task::Poll;

use protocol::futures::StreamExt;
use protocol::futures::{prelude::*, Stream};
use protocol::Event;

pub struct NextVec<'a, T>(pub &'a mut T);

impl<'a, T: Stream<Item = Event> + Unpin> Future for NextVec<'a, T> {
    type Output = Option<Vec<Event>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut vec = Vec::new();
        while let Poll::Ready(option) = self.0.poll_next_unpin(cx) {
            match option {
                Some(event) => vec.push(event),
                None => return Poll::Ready(None),
            }
        }
        Poll::Ready(Some(vec))
    }
}
