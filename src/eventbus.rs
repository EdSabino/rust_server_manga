use std::sync::RwLock;
use std::collections::HashMap;
use std::any::Any;

#[macro_export]
macro_rules! subscribe {
    ($b:expr, $h:expr) => {
        $crate::eventbus::register($b, $h)
    };
}

#[macro_export]
macro_rules! create_eventbus {
    ($b:expr, $name:ty) => {
        $crate::eventbus::create_event_bus::<$name, &str>($b)
    };
}

#[macro_export]
macro_rules! dispatch {
    ($b:expr, $e:expr) => {
        $crate::eventbus::dispatch($b, $e)
    };
}


lazy_static! {
    static ref EVENT_HANDLER_MAP: RwLock<HashMap<String, RwLock<Box<dyn Any + Send + Sync + 'static>>>> = RwLock::new(HashMap::new());
}

pub trait Event: 'static {

}

struct EventHandlers<T: Event>(Vec<Box<dyn Fn(&mut T) + Send + Sync + 'static>>);

impl<T: Event> Default for EventHandlers<T> {
    fn default() -> Self {
        EventHandlers(vec![])
    }
}

pub fn create_event_bus<T: Event, S: Into<String>>(name: S) -> String {
    let name = name.into();
    let mut map = EVENT_HANDLER_MAP.write().unwrap();
    match map.get(&name.clone()) {
        Some(_) => panic!("Key {} already inserted", name),
        None => map.insert(name.clone(), RwLock::new(Box::new(EventHandlers::<T>::default())))
    };
    
    name
}

pub fn register<S: Into<String>, T: Event, H: Fn(&mut T) + Send + Sync + 'static>(name: S, handler: H) {
    let name = name.into();
    let map = EVENT_HANDLER_MAP.read().unwrap();

    match map.get(&name.clone()) {
        Some(value) => {
            let mut writer = value.write().unwrap();
            let handler_vec = writer.downcast_mut::<EventHandlers<T>>().unwrap();
            handler_vec.0.push(Box::new(handler));
        },
        None => panic!("Key {} not found", name)
    };
}

pub fn dispatch<S: Into<String>, T: Event>(name: S, event: &mut T) {
    let name = name.into();
    let map = EVENT_HANDLER_MAP.read().unwrap();

    match map.get(&name.clone()) {
        Some(value) => {
            let mut writer = value.write().unwrap();
            let handler_vec = writer.downcast_mut::<EventHandlers<T>>().unwrap();
            for handler in handler_vec.0.iter() {
                handler(event);
            }
        },
        None => panic!("Key {} not found", name)
    };
}
