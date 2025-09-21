use std::{cell::RefCell, rc::Rc};

use gloo::render::{AnimationFrame, request_animation_frame};
use web_sys::window;

pub struct RequestAnimateFrameLoop {
    _frame: Rc<RefCell<AnimationFrame>>,
}
impl RequestAnimateFrameLoop {
    pub fn new(f: impl Fn(f64) + 'static) -> Self {
        let frame = Rc::new(RefCell::new(request_animation_frame(|_| {})));

        fn helper(t: f64, frame: Rc<RefCell<AnimationFrame>>, f: impl Fn(f64) + 'static) {
            f(t);
            *frame.borrow_mut() = {
                let frame = frame.clone();
                request_animation_frame(|t| helper(t, frame, f))
            };
        }

        *frame.borrow_mut() = {
            let frame = frame.clone();
            request_animation_frame(|t| helper(t, frame, f))
        };

        RequestAnimateFrameLoop { _frame: frame }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WindowDims {
    pub width: f64,
    pub height: f64,
}
impl WindowDims {
    pub fn now() -> Self {
        let window = window().unwrap();
        Self {
            width: window.inner_width().unwrap().as_f64().unwrap(),
            height: window.inner_height().unwrap().as_f64().unwrap(),
        }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}
