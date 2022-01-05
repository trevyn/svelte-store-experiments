use wasm_bindgen::prelude::*;
mod backend;

#[macro_export]
macro_rules! console_log {
  ($($t:tt)*) => (
   #[allow(unsafe_code)]
   #[allow(unused_unsafe)]
   unsafe { log(&format_args!($($t)*).to_string()) }
 )
 }

#[wasm_bindgen]
extern "C" {
 #[wasm_bindgen(js_namespace = console)]
 fn log(s: &str);
}

// store = { subscribe: (subscription: (value: any) => void) => (() => void), set?: (value: any) => void }

// You can create your own stores without relying on svelte/store, by implementing the store contract:

// 1) A store must contain a .subscribe method, which must accept as its argument a subscription function. This subscription function must be immediately and synchronously called with the store's current value upon calling .subscribe. All of a store's active subscription functions must later be synchronously called whenever the store's value changes.

// 2) The .subscribe method must return an unsubscribe function. Calling an unsubscribe function must stop its subscription, and its corresponding subscription function must not be called again by the store.

// For interoperability with RxJS Observables, the .subscribe method is also allowed to return an object with an .unsubscribe method, rather than return the unsubscription function directly. Note however that unless .subscribe synchronously calls the subscription (which is not required by the Observable spec), Svelte will see the value of the store as undefined until it does.

#[wasm_bindgen]
pub fn svelte_store() -> Store {
 Store::new()
}

#[wasm_bindgen]
pub struct Store {
 value: Option<JsValue>,
 subscriptions: Vec<js_sys::Function>,
}

#[wasm_bindgen]
impl Store {
 #[wasm_bindgen(constructor)]
 pub fn new() -> Store {
  Store { value: None, subscriptions: Vec::new() }
 }

 #[wasm_bindgen]
 pub fn subscribe(&mut self, subscription: js_sys::Function) -> JsValue {
  let this = JsValue::null();
  if let Some(value) = &self.value {
   subscription.call1(&this, &value).ok();
  }
  self.subscriptions.push(subscription);

  Closure::wrap(Box::new(move || {
   dbg!("unsubscribe called!!");
  }) as Box<dyn Fn()>)
  .into_js_value()
 }

 #[wasm_bindgen]
 pub fn set(&mut self, value: JsValue) {
  let this = JsValue::null();
  console_log!("set called {:?}", &value);
  for subscription in &mut self.subscriptions {
   subscription.call1(&this, &value).ok();
  }
  self.value = Some(value);
 }
}
