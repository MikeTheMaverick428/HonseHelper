use rmpv::Value;
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Read, Write};

const MSGPACK_LEN_BYTES: usize = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(flatten)]
    pub command: WorkerCommand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum WorkerCommand {
    Ping,
    FindProcess,
    GetViewState {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_scan_bytes: Option<usize>,
    },
    GetVeteranData,
    GetFriendData,
    GetSupportCardData,
    GetUserData,
    GetRaceTeamData,
    GetTrophyData,
    GetCardData,
    Disconnect,
    Quit,
    Exit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerReadyEvent {
    pub event: String,
    pub worker: String,
    pub protocol: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerOkResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    pub ok: bool,
    pub payload: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerErrResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    pub ok: bool,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkerResponse {
    Ready(WorkerReadyEvent),
    Ok(WorkerOkResponse),
    Err(WorkerErrResponse),
}

pub fn respond_ok(id: Option<u64>, payload: Value) -> WorkerResponse {
    WorkerResponse::Ok(WorkerOkResponse {
        id,
        ok: true,
        payload,
    })
}

pub fn respond_err(id: Option<u64>, err: &str) -> WorkerResponse {
    WorkerResponse::Err(WorkerErrResponse {
        id,
        ok: false,
        error: err.to_string(),
    })
}

pub fn ready_event(worker_name: &str) -> WorkerResponse {
    WorkerResponse::Ready(WorkerReadyEvent {
        event: "ready".to_string(),
        worker: worker_name.to_string(),
        protocol: 1,
    })
}

pub fn read_request_msgpack_framed<R: Read>(
    reader: &mut R,
) -> Result<Option<WorkerRequest>, io::Error> {
    let mut len_buf = [0u8; MSGPACK_LEN_BYTES];
    match reader.read_exact(&mut len_buf) {
        Ok(()) => {}
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e),
    }
    let len = u32::from_be_bytes(len_buf) as usize;

    let mut payload = vec![0u8; len];
    reader.read_exact(&mut payload)?;

    let request = rmp_serde::from_slice(&payload)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(Some(request))
}

pub fn write_response_msgpack_framed<W: Write>(
    writer: &mut W,
    response: &WorkerResponse,
) -> io::Result<()> {
    let payload = rmp_serde::to_vec_named(response)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let len = payload.len() as u32;
    writer.write_all(&len.to_be_bytes())?;
    writer.write_all(&payload)?;
    Ok(())
}

pub fn write_msgpack_request_framed<W: Write>(
    writer: &mut W,
    req: &WorkerRequest,
) -> io::Result<()> {
    let payload =
        rmp_serde::to_vec_named(req).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    writer.write_all(&(payload.len() as u32).to_be_bytes())?;
    writer.write_all(&payload)?;
    writer.flush()
}

pub fn parse_msgpack_frame_response(frame: &[u8]) -> Option<WorkerResponse> {
    rmp_serde::from_slice(frame).ok()
}

/// Minimal envelope used to peek just the `id` field from a msgpack response
/// without fully deserializing the payload.
#[derive(Deserialize)]
pub struct ResponseEnvelope {
    #[serde(default)]
    pub id: Option<u64>,
}

pub fn parse_msgpack_frame_id(frame: &[u8]) -> Option<u64> {
    rmp_serde::from_slice::<ResponseEnvelope>(frame).ok()?.id
}

/// Reads a length-prefixed msgpack frame and returns the raw bytes without deserializing.
pub fn read_msgpack_frame_raw<R: Read>(reader: &mut R) -> io::Result<Option<Vec<u8>>> {
    let mut len_buf = [0u8; 4];
    match reader.read_exact(&mut len_buf) {
        Ok(()) => {}
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e),
    }
    let len = u32::from_be_bytes(len_buf) as usize;
    let mut payload = vec![0u8; len];
    reader.read_exact(&mut payload)?;
    Ok(Some(payload))
}

pub fn read_msgpack_response_framed<R: Read>(reader: &mut R) -> io::Result<Option<WorkerResponse>> {
    let mut len_buf = [0u8; 4];
    match reader.read_exact(&mut len_buf) {
        Ok(()) => {}
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e),
    }

    let len = u32::from_be_bytes(len_buf) as usize;
    let mut payload = vec![0u8; len];
    reader.read_exact(&mut payload)?;
    let response = rmp_serde::from_slice::<WorkerResponse>(&payload)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(Some(response))
}

pub fn read_json_response_line(
    line: &str,
) -> std::result::Result<WorkerResponse, serde_json::Error> {
    serde_json::from_str(line)
}

pub fn write_json_request_line<W: Write>(writer: &mut W, req: &WorkerRequest) -> io::Result<()> {
    let line =
        serde_json::to_string(req).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    writer.write_all(line.as_bytes())?;
    writer.write_all(b"\n")?;
    writer.flush()
}

pub fn read_first_non_whitespace<R: BufRead>(reader: &mut R) -> io::Result<Option<u8>> {
    let buf = reader.fill_buf()?;
    Ok(buf.iter().copied().find(|b| !b.is_ascii_whitespace()))
}
