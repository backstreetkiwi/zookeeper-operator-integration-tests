use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use stackable_zookeeper_crd::ZookeeperVersion;
use std::io::{Read, Write};
use std::net::TcpStream;

/// Lists the outstanding sessions and ephemeral nodes. This only works on the leader.
//pub const DUMP: &str = "dump";
/// Print details about serving environment
//pub const ENVIRONMENT: &str = "envi";
/// Shuts down the server. This must be issued from the machine the ZooKeeper server is running on.
//pub const KILL_SERVER: &str = "kill";
/// List outstanding requests
//pub const LIST_REQUESTS: &str = "reqs";
/// Tests if server is running in a non-error state. The server will respond with imok if it is running. Otherwise it will not respond at all.
pub const ARE_YOU_OK: &str = "ruok";
/// Reset statistics returned by stat command.
//pub const RESET_STATISTICS: &str = "srst";
/// Lists statistics about performance and connected clients.
//pub const LIST_STATISTICS: &str = "stat";

/// Positive response for the "ruok" command.
pub const I_AM_OK: &str = "imok";

/// This sends the "four letter word" in order to check if the cluster is ready or to get
/// statistics. We have to differentiate between the ZooKeeper versions.
/// Up to 3.5.2 the standard four letter word can be used.
/// From 3.5.3 onwards we query the admin server via http request.
pub fn send_four_letter_word(
    version: &ZookeeperVersion,
    four_letter_word: &str,
    host: &str,
) -> Result<String> {
    match version {
        ZookeeperVersion::v3_4_14 => send_four_letter_word_to_host(four_letter_word, host),
        ZookeeperVersion::v3_5_8 => send_command_to_admin_server(four_letter_word, host),
    }
}

/// Create a TCP connection to the given host name (format: <host>:<port>) and send the
/// provided 4 letter command (e.g. "ruok") and return the received response.
/// The response is hardcoded to be 4 letters as well, even though some of the four
/// letter commands of ZooKeeper return more data (e.g. stat).
/// This only works until version 3.5.2. With version 3.5.3 this functionality was moved to the
/// admin server. To keep up the four letter words you have to whitelist the required commands
/// in the zoo.cfg via: "4lw.commands.whitelist=*" ("*" for all commands to be whitelisted)
pub fn send_four_letter_word_to_host(four_letter_word: &str, host: &str) -> Result<String> {
    let mut stream = TcpStream::connect(host)?;

    println!("Writing [{}] to [{}]", four_letter_word, host);
    stream.write_all(four_letter_word.as_bytes())?;
    stream.flush()?;

    let mut response = [0u8; 4];
    stream.read(&mut response)?;

    let received = std::str::from_utf8(&response).expect("valid utf8");

    println!("Received: {}", received);
    Ok(received.to_string())
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct AdminServerResponse {
    pub command: String,
    pub error: Option<String>,
}

/// Send a http request to "http:<host>:<port>/commands/<command>
/// It will return a JSON response containing at least:
/// {
///     command: "some_string",
///     error: "some_error"
/// }
/// If no errors occur, "null" (which in serde parses to None) is returned
pub fn send_command_to_admin_server(command: &str, host: &str) -> Result<String> {
    // TODO: Support https
    let url = format!("http://{}/commands/{}", host, command);

    println!("Requesting [{}]", url);
    let mut res = reqwest::blocking::get(&url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let response: AdminServerResponse = serde_json::from_str(&body)?;

    println!("Received: {}", body);

    if response.error.is_none() {
        return Ok(response.command);
    }

    Err(anyhow!(
        "Received error while executing command to admin server: {:?}",
        response.error
    ))
}
