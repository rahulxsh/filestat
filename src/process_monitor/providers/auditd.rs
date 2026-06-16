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
            println!("{line}");
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}