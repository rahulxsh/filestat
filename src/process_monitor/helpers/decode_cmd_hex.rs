use anyhow::{Result,Context};
use hex::decode;
pub fn decode_hex(cmd_hex:&str) -> Result<String> {
    let bytes = decode(cmd_hex).context("Failed to decode command hex")?;

    let command = String::from_utf8(bytes)
        .context("Failed to convert com bytes into string")?;

    Ok(command)
}