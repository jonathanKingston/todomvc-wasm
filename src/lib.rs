#![feature(type_ascription)]
#![feature(option_replace)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

extern crate js_sys;
extern crate web_sys;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::Mutex;

pub mod controller;
pub mod scheduler;
pub mod store;
pub mod view;
use crate::controller::{Controller, ControllerMessage};
use crate::scheduler::Scheduler;
use crate::store::Store;
use crate::view::{View, ViewMessage};

/*
// TODO remove when wasm bindgen supports this
#[wasm_bindgen]
extern "C" {
    type DefinePropertyAttrs;
    #[wasm_bindgen(method, setter, structural)]
    fn set_value(this: &DefinePropertyAttrs, val: &JsValue);
}
*/

pub enum Message {
    Controller(ControllerMessage),
    View(ViewMessage),
}

fn dbg(message: &str) {
    let v = wasm_bindgen::JsValue::from_str(&format!("{}", message));
    web_sys::console::log_1(&v);
}

fn app(name: &str) -> Option<()> {
    use std::borrow::Borrow;
    let mut sched = Rc::new(Scheduler::new());
    let store = Store::new(name)?;
    let controller = Controller::new(store, None, Rc::downgrade(&sched));
    let view = View::new(Rc::downgrade(&sched))?;
    let sch: &Rc<Scheduler> = sched.borrow();
    sch.set_view(view);
    sch.set_controller(controller);
    sched.add_message(Message::Controller(ControllerMessage::AddItem(
        "boop".into(),
    )));
    sched.add_message(Message::Controller(ControllerMessage::SetPage(
        "".to_string(),
    )));
    Some(())
}

#[wasm_bindgen]
pub fn run() {
    app("todos-wasmbindgen");
    /*
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {

            if let Some(store) = Store::new("todos-wasmbindgen") {
                let mut controller = Controller::new(store, None);
                if let Some(mut view) = View::new() {
                    // let template = new Template();
                    //const view = new View(template);

                    let set_page = Closure::wrap(Box::new(move || {
                        if let Some(location) = document.location() {
                            if let Ok(hash) = location.hash() {
                                sched.add_message(Message::Controller(ControllerMessage::SetPage(hash)));
                            }
                        }
                    }) as Box<FnMut()>);
                    let window_et: web_sys::EventTarget = window.into();
                    window_et.add_event_listener_with_callback(
                        "load",
                        set_page.as_ref().unchecked_ref(),
                    );
                    window_et.add_event_listener_with_callback(
                        "hashchange",
                        set_page.as_ref().unchecked_ref(),
                    );
                }
            }
        }
    }
*/
}
