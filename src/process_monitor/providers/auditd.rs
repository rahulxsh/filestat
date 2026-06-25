use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader,BufRead,Seek,SeekFrom};
use anyhow::Result;

pub fn auditd_provider() -> Result<()> {
    let mut file = File::open("/var/log/audit/audit.log")?;

    let mut reader = BufReader::new(file);

    reader.seek(SeekFrom::Start(0))?;
    let mut map:HashMap<u64,Vec<HashMap<String,String>>> = HashMap::new();

    loop {
        let mut line = String::new();

        if reader.read_line(&mut line)? > 0 {
            let fields = parse_fields(&line);
            let event_id = event_id(&line);

            if let Some(record_type) = fields.get("type") {
                if matches!(
                    record_type.as_str(),
                    "SYSCALL" | "EXECVE" | "CWD" | "PATH" | "PROCTITLE"
                ) {
                    if let Some(id) = event_id {
                        map.entry(id).or_default().push(fields)
                    }
                }
            }
        }
        line.clear();

        std::thread::sleep(std::time::Duration::from_millis(100));
        println!("Process Logs:{:?}",map);
    }
}


fn parse_fields(line:&str) -> HashMap<String,String> {
    let mut map = HashMap::new();

    for part in line.split_whitespace() {
        if let Some((key,value)) =  part.split_once("="){
            map.insert(
                key.to_string(),
                value.trim_matches('"').to_string()
            );
        }
    }
    map
}

fn event_id(line: &str) -> Option<u64> {
    let start = line.find("audit(")?;
    let rest = &line[start + 6..];

    let colon = rest.find(':')?;
    let after_colon = &rest[colon + 1..];

    let end = after_colon.find(')')?;

    after_colon[..end].parse().ok()
}