use gloo::render::{request_animation_frame,AnimationFrame};
use std::{
    future::Future,
    cell::RefCell,
    pin::Pin,
    rc::Rc,
    task::Poll
};

pub struct  RequestAnimationFrame {
    raf: Option<AnimationFrame>,
    delta: Rc<RefCell<Option<f64>>>
}

impl RequestAnimationFrame {
    fn new() -> Self {
        Self {
            raf: None,
            delta: Rc::new(RefCell::new(None))
        }
    }
}
impl Future for RequestAnimationFrame {
    type Output = f64;
    fn poll(self: Pin<&mut Self>,cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        match this.delta.take() {
            None => {
                this.raf = Some(request_animation_frame({
                    let waker = cx.waker().clone();
                    let delta = this.delta.clone();
                    move |d| {
                        *delta.borrow_mut() = Some(d);
                        waker.wake();
                    }
                }));
                Poll::Pending
            }
            Some(delta) => {
                Poll::Ready(delta)
            }
        }
    }
}

pub async fn wait_request_animation_frame() -> f64 {
    RequestAnimationFrame::new().await
}