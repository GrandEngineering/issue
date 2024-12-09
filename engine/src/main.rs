use enginelib::Registry;
use enginelib::{api::EngineAPI, events};
use libloading::os::unix::{Library, Symbol};
use std::sync::Arc;

// Main project
fn main() {
    let start_event_id = ("core".to_string(), "onstartevent".to_string());
    //Instatiates an API instance
    let mut api = EngineAPI::default();

    //registers a lib
    unsafe {
        let lib = Library::new("target/debug/libengine_core.so").unwrap();
        let run: Symbol<unsafe extern "Rust" fn(reg: &mut EngineAPI)> = lib.get(b"run").unwrap();
        run(&mut api);
    }

    // Registers an Event type
    api.event_bus.event_registry.register(
        Arc::new(events::start_event::StartEvent {
            modules: api.modules.values().cloned().collect(),
            id: start_event_id.clone(),
            cancelled: false,
        }),
        start_event_id.clone(),
    );

    //Segfaults after below
    api.event_bus.handle(
        start_event_id.clone(),
        &mut events::start_event::StartEvent {
            cancelled: false,
            id: start_event_id.clone(),
            modules: api.modules.values().cloned().collect(),
        },
    );
}
