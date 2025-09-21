use std::{cell::RefCell, rc::Rc};

use gloo::render::{AnimationFrame, request_animation_frame};
use sycamore::prelude::{Signal, create_signal};
use web_sys::window;

pub struct RequestAnimateFrameLoop {
    _frame: Signal<AnimationFrame>,
}
impl RequestAnimateFrameLoop {
    pub fn new(f: impl Fn(f64) + 'static) -> Self {
        let frame = create_signal(request_animation_frame(|_| panic!()));

        fn helper(t: f64, frame: Signal<AnimationFrame>, f: impl Fn(f64) + 'static) {
            f(t);
            frame.set(request_animation_frame(move |t| helper(t, frame, f)));
        }

        frame.set(request_animation_frame(move |t| helper(t, frame, f)));

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
