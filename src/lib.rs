//! # TODO MVC
//!
//! A [TODO MVC](https://todomvc.com/) implementation written using [web-sys](https://rustwasm.github.io/wasm-bindgen/web-sys/overview.html)
#![feature(type_ascription)]
#![feature(option_replace)]
#![warn(missing_docs)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate js_sys;
extern crate web_sys;
use std::rc::Rc;

/// Controller of the program
pub mod controller;
/// Element wrapper to the DOM
pub mod element;
/// Schedule messages to the Controller and View
pub mod scheduler;
/// Stores items into localstorage
pub mod store;
/// Handles constructing template strings from data
pub mod template;
/// Presentation layer
pub mod view;
use crate::controller::{Controller, ControllerMessage};
use crate::scheduler::Scheduler;
use crate::store::Store;
use crate::view::{View, ViewMessage};

/// Message wrapper enum used to pass through the scheduler to the Controller or View
pub enum Message {
    /// Message wrapper to send to the controller
    Controller(ControllerMessage),
    /// Message wrapper to send to the view
    View(ViewMessage),
}

/// Used for debugging to the console
pub fn dbg(message: &str) {
    let v = wasm_bindgen::JsValue::from_str(&message.to_string());
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

/// Entry point into the program from JavaScript
#[wasm_bindgen]
pub fn run() {
    app("todos-wasmbindgen");
}
