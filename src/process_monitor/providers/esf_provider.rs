use crate::process_monitor::process::{ProcessEvent, ProcessExecEvent, ProcessExitEvent};
use anyhow::Result;
use endpoint_sec::sys::es_event_type_t;
use endpoint_sec::{Client, Event};
use nix::libc::uid_t;
use nix::unistd::{Uid, User};
use std::time::Duration;


#[allow(dead_code)]
fn get_os_version() -> Vec<u64> {
    let info = os_info::get();

    let version = info.version().to_string();

    let v: Vec<u64> = version
        .split(".")
        .map(|vs| vs.parse::<u64>().unwrap_or(0))
        .collect();

    v
}

fn username_from_uid(id: uid_t) -> Option<String> {
    let uid = Uid::from_raw(id);

    let user = User::from_uid(uid).ok()??.name.to_string();

    Some(user)
}

pub fn esf() -> Result<()> {
    let mut client = Client::new(|_client, message| match message.event() {
        Some(Event::NotifyExec(exec)) => {
            let target = exec.target();
            let uid = target.audit_token().euid();

            let process = ProcessExecEvent {
                timestamp: message.time(),
                pid: target.audit_token().pid() as u32,
                ppid: target.ppid() as u32,
                uid: uid.to_string(),
                user: username_from_uid(uid),
                executable_path: target.executable().path().to_string_lossy().into_owned(),
                command_line: exec
                    .args()
                    .map(|val| val.to_string_lossy().into_owned())
                    .collect::<Vec<_>>(),
            };

            println!("PROCESS:{:?}\n\n", process);
            let _event  = ProcessEvent::Start(process);
        }
        Some(Event::NotifyExit(exit)) => {
            let process = message.process();
            let process_exit_event = ProcessExitEvent {
                timestamp:message.time(),
                pid:process.audit_token().pid() as u32,
                exit_code:exit.stat()
            };
            println!("EXIT EVENT: {:?}", process_exit_event);
            let _event  = ProcessEvent::Exit(process_exit_event);
        }
        _ => {}
    })?;

    let event_types = [
        es_event_type_t::ES_EVENT_TYPE_NOTIFY_EXEC,
        es_event_type_t::ES_EVENT_TYPE_NOTIFY_EXIT,
    ];

    client.subscribe(&event_types)?;

    loop {
        //TODO:Remove this after process event creation done
        std::thread::sleep(Duration::from_secs(2))
    }
}
