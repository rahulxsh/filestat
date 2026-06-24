use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader,BufRead,Seek,SeekFrom};
use anyhow::Result;

pub fn auditd_provider() -> Result<()> {
    let mut file = File::open("/var/log/audit/audit.log")?;

    let mut reader = BufReader::new(file);

    reader.seek(SeekFrom::End(0));

    loop {
        let mut line = String::new();

        if reader.read_line(&mut line)? > 0 {
            let fields = parse_fields(&line);
            println!("Fields is:{:?}",fields);
        }
        line.clear();

        std::thread::sleep(std::time::Duration::from_millis(100));
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