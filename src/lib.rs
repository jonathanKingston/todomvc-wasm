#![feature(type_ascription)]
#![feature(option_replace)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

extern crate js_sys;
extern crate web_sys;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use std::sync::Mutex;

mod controller;
mod store;
mod view;
use crate::controller::{Controller, ControllerMessage};
use crate::store::Store;
use crate::view::{View, ViewMessage};


pub enum Message {
  Controller(ControllerMessage),
  View(ViewMessage)
}

fn dbg(message: &str) {
    let v = wasm_bindgen::JsValue::from_str(&format!("{}", message));
    web_sys::console::log_1(&v);
}

/*
pub struct App {
    scheduler: RefCell<Option<Scheduler>>,
}

impl App {
    fn new(store: String) -> Option<App> {
        let store = Store::new("todos-wasmbindgen").Ok()?;
        let sched = Scheduler::new(app);
        let controller = Controller::new(store, None);
        let view = View::new().ok()?;
        let app = App {
            controller: RefCell::new(None),
            view: RefCell::new(None),
            scheduler: RefCell::new(None),
        };
        app 
    }
}
*/

pub struct Scheduler {
    controller: Rc<RefCell<Option<Controller>>>,
    view: Rc<RefCell<Option<View>>>,
    //app: RefCell<Option<App>>,
    events: RefCell<Vec<Message>>,
    running: RefCell<bool>
}

impl Scheduler {
    fn new() -> Scheduler {
        Scheduler {
            //app: Box::new(app),
            controller: Rc::new(RefCell::new(None)),
            view: Rc::new(RefCell::new(None)),
            events: RefCell::new(Vec::new()),
            running: RefCell::new(false)
        }
    }
    fn set_controller(&self, controller: Controller) {
        let mut controller_data = self.controller.borrow_mut();
        controller_data.replace(controller);
/*
        controller_data.replace(Rc::downgrade(&controller));
        self.controller = Some(controller);
*/
    }
    fn set_view(&self, view: View) {
        let mut view_data = self.view.borrow_mut();
        view_data.replace(view);
/*
        self.view = Some(view);
*/
    }
    fn setup(&self) {
        //self.controller.borrow_mut().unwrap().set_sched(&self);
/*
        let schedrc = Rc::new(self);
                     if let Some(ref mut controller) = *sched.controller.borrow_mut() {
                       controller.set_sched(Rc::downgrade(&schedrc));
*/
        if let Some(ref mut controller) = *self.controller.borrow_mut() {
      //      controller.set_sched(Rc::downgrade(&Rc::new(*self)));
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
                    if let Some(ref mut ag) = *self.controller.borrow_mut() { // TODO use try_borrow
                        ag.call(e);
                    }
                },
                Message::View(e) => {
                    if let Some(ref mut ag) = *self.view.borrow_mut() { // TODO use try_borrow
                        ag.call(e);
                    }
                },
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
/*
    if let Some(r) = Rc::get_mut(&mut sched) {
panic!("sss rr");
        r.set_controller(controller);
        r.set_view(view);
    }
*/
/*
    let x_sched = Rc::into_raw(sched);
    unsafe {
        let mut x = Rc::from_raw(x_sched);
        x.set_controller(controller);
    }
*/
    //sched.set_controller(controller);
    //sched.set_view(view);
    sched.add_message(Message::Controller(ControllerMessage::AddItem("boop".into())));
    sched.add_message(Message::Controller(ControllerMessage::SetPage("".to_string())));
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
//let crc = Rc::new(controller);
//let vrc = Rc::new(view);

                    sched.set_controller(controller);
                    sched.set_view(view);
                    sched.add_message(Message::Controller(ControllerMessage::AddItem("boop".into())));
/*
                    {
                    let schedrc = Rc::new(sched);
                     if let Some(ref mut controller) = *sched.controller.borrow_mut() {
                       controller.set_sched(Rc::downgrade(&schedrc));
                     }
                    }
*/
                    sched.setup();

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
*/
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
