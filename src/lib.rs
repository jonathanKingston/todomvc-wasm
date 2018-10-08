#![feature(type_ascription)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

extern crate js_sys;
extern crate web_sys;
use std::rc::Rc;
use std::sync::Mutex;

mod controller;
mod store;
mod view;
use crate::controller::{Controller, ControllerMessage};
use crate::store::Store;
use crate::view::View;


use std::cell::RefCell;

enum ViewMessage {
}

enum Message {
  Controller(ControllerMessage),
  View(ViewMessage)
}

struct Scheduler {
    controller: RefCell<Controller>,
    view: RefCell<View>,
    events: RefCell<Vec<Message>>,
    running: RefCell<bool>
}

fn dbg(message: &str) {
    let v = wasm_bindgen::JsValue::from_str(&format!("{}", message));
      web_sys::console::log_1(&v);
}

impl Scheduler {
    fn new(view: View, controller: Controller) -> Scheduler {
        Scheduler {
            controller: RefCell::new(controller),
            view: RefCell::new(view),
            events: RefCell::new(Vec::new()),
            running: RefCell::new(false)
        }
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
        if running == false {
            self.run();
        }
    }
    fn run(&self) {
        dbg("run");
        let mut events_len = 0;
        {
            events_len = self.events.borrow().len().clone(); // TODO use try_borrow
        }
                    let v = wasm_bindgen::JsValue::from_str(&format!("{}", "run 1"));
                    web_sys::console::log_1(&v);
                    let v = wasm_bindgen::JsValue::from_str(&format!("{}", "run 2"));
                    web_sys::console::log_1(&v);
        if events_len == 0 {
                    let v = wasm_bindgen::JsValue::from_str(&format!("{}", "run 3"));
                    web_sys::console::log_1(&v);
            let mut running = self.running.borrow_mut();
            *running = false;
        } else {
                    let v = wasm_bindgen::JsValue::from_str(&format!("{}", "run 4"));
                    web_sys::console::log_1(&v);
            {
                let mut running = self.running.borrow_mut();
                *running = false;
            }
                    let v = wasm_bindgen::JsValue::from_str(&format!("{}", "run 5"));
                    web_sys::console::log_1(&v);
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
                    let mut controller = self.controller.borrow_mut(); // TODO use try_borrow
                    controller.call(e);
                },
                _ => println!("unsupported:"),
                //Message::View(e) => self.view.call(e),
            }
            self.run();
        } else {
            let mut running = self.running.borrow_mut();
            *running = false;
        }
    }
}

#[wasm_bindgen]
pub fn run() {
    //   App::new("todos-wasmbindgen");
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(store) = Store::new("todos-wasmbindgen") {
                let mut controller = Controller::new(store, None);
                if let Some(mut view) = View::new() {
                    // let template = new Template();
                    //const view = new View(template);

                    let sched = Scheduler::new(view, controller);
                    sched.add_message(Message::Controller(ControllerMessage::AddItem("boop".into())));
/*
                    controller.set_sched(&sched);
*/

                    //let mut controller = Controller::new(store, view);
                    //view.set_controller(Box::new(&controller));
                    let v = wasm_bindgen::JsValue::from_str(&format!("{}", "hay"));
                    web_sys::console::log_1(&v);
                   // controller.set_view(view);
                   // controller.set_page("".to_string());
                    sched.add_message(Message::Controller(ControllerMessage::SetPage("".to_string())));
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
}

/*
struct App<'app> {
  controller: Controller<'app>,
  view: View<'app>,
  store: Store,
}

impl <'app> App <'app> {
    fn new(name: &str) -> Option<App> {
        let view = View::new();
        let store = Store::new(name);
        if let Some(store) = store {
            let controller = Controller::new(&mut store, None);
            if let Some(view) = view {
                return Some(App {
                  controller,
                  view,
                  store
                });
            }
        }
        None
    }
}
*/
