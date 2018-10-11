#![feature(type_ascription)]
#![feature(option_replace)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate js_sys;
extern crate web_sys;
use std::rc::Rc;

pub mod controller;
pub mod scheduler;
pub mod store;
pub mod template;
pub mod view;
use crate::controller::{Controller, ControllerMessage};
use crate::scheduler::Scheduler;
use crate::store::Store;
use crate::view::{View, ViewMessage};

pub enum Message {
    Controller(ControllerMessage),
    View(ViewMessage),
}

pub fn dbg(message: &str) {
    let v = wasm_bindgen::JsValue::from_str(&format!("{}", message));
    web_sys::console::log_1(&v);
}

fn app(name: &str) -> Option<()> {
    use std::borrow::Borrow;
    let sched = Rc::new(Scheduler::new());
    let store = Store::new(name)?;
    let controller = Controller::new(store, Rc::downgrade(&sched));
    let mut view = View::new(sched.clone())?;
    let sch: &Rc<Scheduler> = sched.borrow();
    view.init();
    sch.set_view(view);
    sch.set_controller(controller);
    sched.add_message(Message::Controller(ControllerMessage::SetPage(
        "".to_string(),
    )));
    Some(())
}

#[wasm_bindgen]
pub fn run() {
    app("todos-wasmbindgen");
}
