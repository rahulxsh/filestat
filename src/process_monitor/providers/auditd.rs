// use std::collections::HashMap;
// use std::fs::File;
// use std::io::{BufReader,BufRead,Seek,SeekFrom};
// use anyhow::Result;
//
//
// pub fn auditd_provider() -> Result<()> {
//     let file = File::open("/var/log/audit/audit.log")?;
//
//     let mut reader = BufReader::new(file);
//
//     reader.seek(SeekFrom::End(0))?;
//     let mut map:HashMap<u64,Vec<AuditRecord>> = HashMap::new();
//
//     loop {
//         let mut line = String::new();
//
//         if reader.read_line(&mut line)? > 0 {
//
//             if let Some((event_id,record)) = parse_fields(&line) {
//                 if matches!(
//                     record.audit_type.as_str(),
//                     "SYSCALL" | "EXECVE" | "CWD" | "PATH" | "PROCTITLE"
//                 ) {
//                         map.entry(event_id).or_default().push(record)
//                 }
//             }
//         }
//         line.clear();
//         println!("MAP LENGTH: {}",map.len());
//
//         std::thread::sleep(std::time::Duration::from_millis(100));
//     }
// }
//
// #[derive(Debug)]
// struct AuditRecord {
//     audit_type:String,
//     fields:HashMap<String,String>
// }
//
// fn parse_fields(line:&str) -> Option<(u64, AuditRecord)> {
//     let mut map = HashMap::new();
//
//
//     let event_id = event_id(line)?;
//
//     for part in line.split_whitespace() {
//         if let Some((key,value)) =  part.split_once("="){
//             map.insert(
//                 key.to_string(),
//                 value.trim_matches('"').to_string()
//             );
//         }
//     }
//
//     let record_type = map.get("type").cloned().unwrap_or_else(|| String::from("UNKNOWN"));
//
//     let record = AuditRecord {
//         audit_type:record_type,
//         fields:map
//     };
//
//     Some((event_id,record))
// }
//
// fn event_id(line: &str) -> Option<u64> {
//     let start = line.find("audit(")?;
//     let rest = &line[start + 6..];
//
//     let colon = rest.find(':')?;
//     let after_colon = &rest[colon + 1..];
//
//     let end = after_colon.find(')')?;
//
//     after_colon[..end].parse().ok()
// }

use netlink_sys::{protocols::NETLINK_ROUTE, Socket, SocketAddr};
use anyhow::Result;

pub fn netlink_audit() -> Result<()> {
    let mut socket = Socket::new(NETLINK_ROUTE)?;

    let kernal_addr = SocketAddr::new(0,0);

    socket.bind(&kernal_addr);

    let mut buffer = vec![0; 8192];

    loop {
        let data = socket.recv_from(&mut buffer,0)?;

        println!("Received bytes for socket:{:?}",data.0);
    }
}