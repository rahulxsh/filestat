use std::time::Duration;
use endpoint_sec::{Event, Client, Message, EventExec};
use anyhow::Result;
use endpoint_sec::sys::RespondError::EventType;
use endpoint_sec::version::{is_version_or_more, set_runtime_version};
use endpoint_sec::sys::es_event_type_t;

fn get_os_version() -> Vec<u64> {
    //TODO:First GET the entitlement from apple developer
    let info  = os_info::get();

    let version = info.version().to_string();

    let v:Vec<u64> = version
        .split(".")
        .map(|vs| vs.parse::<u64>()
            .unwrap_or(0))
        .collect();

    v
}

pub fn esf() -> Result<()> {
    let v = get_os_version();

    unsafe  {
        set_runtime_version(v[0],v[1],v[2]);
    }

    let mut client = Client::new(|mut client, message| {
        if let Some(event) = message.event() {
            println!("{:?}",event);
        }
    } )?;

    let event_types = [
        es_event_type_t::ES_EVENT_TYPE_NOTIFY_EXIT,
        es_event_type_t::ES_EVENT_TYPE_AUTH_EXEC
    ];

    client.subscribe(&event_types)?;

    loop {
        std::thread::sleep(Duration::from_secs(2))
    }
}