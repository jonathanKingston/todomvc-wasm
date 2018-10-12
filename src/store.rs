use js_sys::JSON;
use wasm_bindgen::prelude::*;
//use crate::DefinePropertyAttrs;
// TODO remove
fn dbg(mssg: &str) {
    let v = wasm_bindgen::JsValue::from_str(&format!("{}", mssg));
    web_sys::console::log_1(&v);
}

/// Stores items into localstorage
pub struct Store {
    local_storage: web_sys::Storage,
    data: Option<ItemList>,
    name: String,
}

impl Store {
    /// Creates a new store with `name` as the localstorage value name
    pub fn new(name: &str) -> Option<Store> {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(local_storage)) = window.local_storage() {
                Some(Store {
                    local_storage,
                    data: None,
                    name: String::from(name),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Read the local ItemList from localStorage.
    /// Returns an &Option<ItemList> of the stored database
    /// Caches the store into `self.data` to reduce calls to JS
    ///
    /// Uses mut here as the return is something we might want to manipulate
    ///
    fn get_local_storage(&mut self) -> &Option<ItemList> {
        let mut item_list = ItemList::new();
        if let Some(ref _data) = self.data {
            return &self.data;
        }
        if let Ok(Some(value)) = self.local_storage.get_item(&self.name) {
            if let Ok(data) = JSON::parse(&value) {
                if let Ok(Some(iter)) = js_sys::try_iter(&data) {
                    for item in iter {
                        if let Ok(item) = item {
                            let item: Option<&js_sys::Array> = wasm_bindgen::JsCast::dyn_ref(&item);
                            if let Some(item) = item {
                                if let Some(title) = item.shift().as_string() {
                                    if let Some(completed) = item.shift().as_bool() {
                                        if let Some(id) = item.shift().as_f64() {
                                            let mut temp_item = Item {
                                                title,
                                                completed,
                                                id: format!("{}", id),
                                            };
                                            item_list.push(temp_item);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self.data = Some(item_list);
        &self.data
    }

    /// Write the local ItemList to localStorage.
    ///
    /// todos `ItemList` is the items to store
    fn set_local_storage(&mut self, todos: ItemList) {
        let array = js_sys::Array::new();
        for item in todos.iter() {
            let mut child = js_sys::Array::new();
            let s = String::from(item.title.clone());
            child.push(&JsValue::from(&s));
            child.push(&JsValue::from(item.completed));
            child.push(&JsValue::from(item.id.to_string()));

            array.push(&JsValue::from(child));
        }
        if let Ok(storage_string) = JSON::stringify(&JsValue::from(array)) {
            let storage_string: String = storage_string.to_string().into();
            dbg(&storage_string);
            self.local_storage
                .set_item(&self.name, storage_string.as_str());
            self.data = Some(todos);
        }
    }

    /// Find items with properties matching those on query.
    /// `ItemQuery` query Query to match
    ///
    /// ```
    ///  let data = db.find(ItemQuery::Completed {completed: true});
    ///	 // data will contain items whose completed properties are true
    /// ```
    pub fn find(&mut self, query: ItemQuery) -> Option<ItemListSlice> {
        self.get_local_storage();
        if let Some(ref todos) = self.data {
            Some(todos.iter().filter(|todo| query.matches(*todo)).collect())
        } else {
            None
        }
    }

    /// Update an item in the Store.
    ///
    /// `ItemUpdate` update Record with an id and a property to update
    pub fn update(&mut self, update: ItemUpdate) {
        let id = update.id();
        self.get_local_storage();
        if let Some(todos) = self.data.take() {
            let todos = todos.into_iter();

            let todos = todos
                .map(|mut todo| {
                    if id == todo.id {
                        todo.update(&update);
                    }
                    todo
                }).collect();
            self.set_local_storage(todos);
        }
    }

    /// Insert an item into the Store.
    ///
    /// `Item` item Item to insert
    pub fn insert(&mut self, item: Item) {
        self.get_local_storage();
        if let Some(mut todos) = self.data.take() {
            todos.push(item);
            self.set_local_storage(todos);
        }
    }

    /// Remove items from the Store based on a query.
    /// query is an `ItemQuery` query Query matching the items to remove
    pub fn remove(&mut self, query: ItemQuery) {
        self.get_local_storage();
        if let Some(todos) = self.data.take() {
            let todos = todos
                .into_iter()
                .filter(|todo| query.matches(todo))
                .collect();
            self.set_local_storage(todos);
        }
    }

    /// Count total, active, and completed todos.
    pub fn count(&mut self) -> Option<(usize, usize, usize)> {
        self.find(ItemQuery::EmptyItemQuery).map(|data| {
            let total = data.length();

            let mut completed = 0;
            for item in data.iter() {
                if item.completed {
                    completed += 1;
                }
            }
            (total, total - completed, completed)
        })
    }
}

/// Represents a todo item
pub struct Item {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

impl Item {
    pub fn update(&mut self, update: &ItemUpdate) {
        match update {
            ItemUpdate::Title { title, .. } => {
                self.title = title.to_string();
            }
            ItemUpdate::Completed { completed, .. } => {
                self.completed = *completed;
            }
        }
    }
}

pub trait ItemListTrait<T> {
    fn new() -> Self;
    fn get(&self, i: usize) -> Option<&T>;
    fn length(&self) -> usize;
    fn push(&mut self, item: T);
    fn iter(&self) -> std::slice::Iter<T>;
}

/// A list of todo items
pub struct ItemList {
    list: Vec<Item>,
}
impl ItemList {
    fn into_iter(self) -> std::vec::IntoIter<Item> {
        self.list.into_iter()
    }
}
impl ItemListTrait<Item> for ItemList {
    fn new() -> ItemList {
        ItemList { list: Vec::new() }
    }
    fn get(&self, i: usize) -> Option<&Item> {
        self.list.get(i)
    }
    fn length(&self) -> usize {
        self.list.len()
    }
    fn push(&mut self, item: Item) {
        self.list.push(item)
    }
    fn iter(&self) -> std::slice::Iter<Item> {
        self.list.iter()
    }
}
use std::iter::FromIterator;
impl<'a> FromIterator<Item> for ItemList {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> Self {
        let mut c = ItemList::new();
        for i in iter {
            c.push(i);
        }

        c
    }
}

/// A borrowed set of Items filtered from the store
pub struct ItemListSlice<'a> {
    list: Vec<&'a Item>,
}
impl<'a> ItemListSlice<'a> {
    /*
TODO implement?
    fn new(boxed_slice: Box<[&'a Item]>) -> ItemListSlice<'a> {
        ItemListSlice { list: boxed_slice }
    }
*/
}
impl<'a> ItemListTrait<&'a Item> for ItemListSlice<'a> {
    fn new() -> ItemListSlice<'a> {
        ItemListSlice { list: Vec::new() }
    }
    fn get(&self, i: usize) -> Option<&&'a Item> {
        self.list.get(i)
    }
    fn length(&self) -> usize {
        self.list.len()
    }
    fn push(&mut self, item: &'a Item) {
        self.list.push(item)
    }
    fn iter(&self) -> std::slice::Iter<&'a Item> {
        self.list.iter()
    }
}

impl<'a> FromIterator<&'a Item> for ItemListSlice<'a> {
    fn from_iter<I: IntoIterator<Item = &'a Item>>(iter: I) -> Self {
        let mut c = ItemListSlice::new();
        for i in iter {
            c.push(i);
        }
        c
    }
}

impl<'a> Into<ItemList> for ItemListSlice<'a> {
    fn into(self) -> ItemList {
        let mut i = ItemList::new();
        let items = self.list.into_iter();
        for j in items {
            // TODO neaten this cloning?
            let item = Item {
                id: j.id.clone(),
                completed: j.completed,
                title: j.title.clone(),
            };
            i.push(item);
        }
        i
    }
}

/// Represents a search into the store
pub enum ItemQuery {
    Id { id: String },
    Completed { completed: bool },
    EmptyItemQuery,
}

impl ItemQuery {
    fn matches(&self, item: &Item) -> bool {
        match *self {
            ItemQuery::EmptyItemQuery => true,
            ItemQuery::Id { ref id } => &item.id == id,
            ItemQuery::Completed { completed } => item.completed == completed,
        }
    }
}

pub enum ItemUpdate {
    Title { id: String, title: String },
    Completed { id: String, completed: bool },
}

impl ItemUpdate {
    fn id(&self) -> String {
        match self {
            ItemUpdate::Title { id, .. } => id.clone(),
            ItemUpdate::Completed { id, .. } => id.clone(),
        }
    }
}
