#![feature(type_ascription)]
#![feature(option_replace)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

extern crate js_sys;
extern crate web_sys;
use std::rc::{Weak, Rc};
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

fn dbg(message: &str) {
    let v = wasm_bindgen::JsValue::from_str(&format!("{}", message));
      web_sys::console::log_1(&v);
}

pub struct Scheduler {
    controller: RefCell<Option<Weak<Controller>>>,
    view: RefCell<Option<Weak<View>>>,
    events: RefCell<Vec<Message>>,
    running: RefCell<bool>
}

impl Scheduler {
    fn new() -> Scheduler {
        Scheduler {
            controller: RefCell::new(None),
            view: RefCell::new(None),
            events: RefCell::new(Vec::new()),
            running: RefCell::new(false)
        }
    }
    fn set_controller(&self, controller: Rc<Controller>) {
        let mut controller_data = self.controller.borrow_mut();
        controller_data.replace(Rc::downgrade(&controller));
    }
    fn set_view(&self, view: Rc<View>) {
        let mut view_data = self.view.borrow_mut();
        view_data.replace(Rc::downgrade(&view));
    }
    fn setup(&self) {
        //self.controller.borrow_mut().unwrap().set_sched(&self);
        //if let Some(ref mut controller) = *self.controller.borrow_mut() {
            //controller.set_sched(&self);
        //}
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
                *running = true;
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
    // controller: RefCell<Option<Weak<Controller>>>,
                    if let Some(ref ag) = *self.controller.borrow_mut() { // TODO use try_borrow
                        if let Some(ref mut controller) = *ag.upgrade() {
                            controller.call(e);
                        }
                    }
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
    let sched = Rc::new(Scheduler::new());
    //   App::new("todos-wasmbindgen");
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(store) = Store::new("todos-wasmbindgen") {
                let mut controller = Controller::new(store, None, sched.clone());
                if let Some(mut view) = View::new() {
                    // let template = new Template();
                    //const view = new View(template);
let crc = Rc::new(controller);
let vrc = Rc::new(view);

                    sched.set_controller(crc);
                    sched.set_view(vrc);
                    sched.add_message(Message::Controller(ControllerMessage::AddItem("boop".into())));
                    //sched.setup();

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
