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
pub mod store;
pub mod view;
use crate::controller::{Controller, ControllerMessage};
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

pub struct Scheduler {
    controller: Rc<RefCell<Option<Controller>>>,
    view: Rc<RefCell<Option<View>>>,
    events: RefCell<Vec<Message>>,
    running: RefCell<bool>,
}

impl Scheduler {
    fn new() -> Scheduler {
        Scheduler {
            controller: Rc::new(RefCell::new(None)),
            view: Rc::new(RefCell::new(None)),
            events: RefCell::new(Vec::new()),
            running: RefCell::new(false),
        }
    }
    fn set_controller(&self, controller: Controller) {
        let mut controller_data = self.controller.borrow_mut();
        controller_data.replace(controller);
    }
    fn set_view(&self, view: View) {
        let mut view_data = self.view.borrow_mut();
        view_data.replace(view);
    }
    fn add_message(&self, message: Message) {
        let v = wasm_bindgen::JsValue::from_str(&format!("{}", "got message"));
        web_sys::console::log_1(&v);
        let mut running = false;
        {
            running = self.running.borrow().clone();
        }
        {
            let mut events = self.events.borrow_mut(); // TODO use try_borrow
            events.push(message);
        }
        if !running {
            self.run();
        }
    }
    fn run(&self) {
        dbg("run");
        let mut events_len = 0;
        {
            events_len = self.events.borrow().len().clone(); // TODO use try_borrow
        }
        dbg("run 1");
        if events_len == 0 {
            dbg("run 3");
            let mut running = self.running.borrow_mut();
            *running = false;
        } else {
            dbg("run 4");
            {
                let mut running = self.running.borrow_mut();
                *running = true;
            }
            dbg("run 5");
            self.next_message();
        }
    }
    fn next_message(&self) {
        let mut event = None;
        {
            let mut events = self.events.borrow_mut(); // TODO use try_borrow
            event = events.pop();
        }
        if let Some(event) = event {
            match event {
                Message::Controller(e) => {
                    if let Some(ref mut ag) = *self.controller.borrow_mut() {
                        // TODO use try_borrow
                        ag.call(e);
                    }
                }
                Message::View(e) => {
                    if let Some(ref mut ag) = *self.view.borrow_mut() {
                        // TODO use try_borrow
                        ag.call(e);
                    }
                }
            }
            self.run();
        } else {
            let mut running = self.running.borrow_mut();
            *running = false;
        }
    }
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
