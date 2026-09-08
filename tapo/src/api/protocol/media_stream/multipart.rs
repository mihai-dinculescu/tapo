//! Multipart framing of the media stream.
//!
//! After the handshake, both directions of the stream carry a sequence of
//! parts. A part is a delimiter line, header lines, an empty line, exactly
//! `Content-Length` bytes of body, and a trailing CRLF. The client writes
//! parts under the `--client-stream-boundary--` boundary and the hub answers
//! under `--device-stream-boundary--`.
//!
//! The Tapo app frames the parts by hand rather than with a generic multipart
//! parser, and the reader here mirrors its behaviour: any line that contains
//! the device boundary is a delimiter, empty lines between parts are skipped,
//! header lines without a colon are ignored, and the body length always comes
//! from `Content-Length`.

use anyhow::{Context, bail};

/// The delimiter line the client writes before each of its parts: two dashes
/// followed by the `--client-stream-boundary--` boundary.
const CLIENT_DELIMITER: &str = "----client-stream-boundary--";
const DEVICE_BOUNDARY: &str = "--device-stream-boundary--";
/// Upper bound on a single part's body, so that a misbehaving peer cannot
/// grow the read buffer without bound. Media parts are small MPEG-TS chunks.
const MAX_BODY_SIZE: usize = 8 * 1024 * 1024;
/// Upper bound on the bytes buffered while waiting for a complete part.
const MAX_BUFFER_SIZE: usize = MAX_BODY_SIZE + 64 * 1024;

/// A part received from the hub.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Part {
    /// Header names are lower-cased.
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Part {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(header, _)| header == name)
            .map(|(_, value)| value.as_str())
    }

    pub fn content_type(&self) -> Option<&str> {
        self.header("content-type")
    }

    pub fn is_json(&self) -> bool {
        self.content_type()
            .map(|content_type| {
                content_type
                    .trim_start()
                    .to_ascii_lowercase()
                    .starts_with("application/json")
            })
            .unwrap_or(false)
    }
}

/// Something the hub sent: either a part or the closing delimiter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Frame {
    Part(Part),
    End,
}

/// Encodes one client part. `Content-Length` is added after the given
/// headers.
pub(super) fn encode_client_part(headers: &[(&str, &str)], body: &[u8]) -> Vec<u8> {
    let mut part = Vec::with_capacity(body.len() + 256);
    part.extend_from_slice(CLIENT_DELIMITER.as_bytes());
    part.extend_from_slice(b"\r\n");
    for (name, value) in headers {
        part.extend_from_slice(format!("{name}: {value}\r\n").as_bytes());
    }
    part.extend_from_slice(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes());
    part.extend_from_slice(body);
    part.extend_from_slice(b"\r\n");
    part
}

/// Incremental parser of the parts the hub sends.
///
/// Bytes are appended to [`PartParser::buffer_mut`] as they arrive and
/// complete frames are taken out with [`PartParser::next_frame`], so reads
/// can be interrupted (e.g. to send a heartbeat) without losing a partially
/// received part.
#[derive(Debug)]
pub(super) struct PartParser {
    buffer: Vec<u8>,
}

impl PartParser {
    /// `initial` holds bytes that were received ahead of the parser, e.g.
    /// together with the HTTP response head.
    pub fn new(initial: Vec<u8>) -> Self {
        Self { buffer: initial }
    }

    pub fn buffer_mut(&mut self) -> &mut Vec<u8> {
        &mut self.buffer
    }

    /// Returns the next complete frame, or `None` when more bytes are needed.
    pub fn next_frame(&mut self) -> anyhow::Result<Option<Frame>> {
        let mut cursor = 0;

        // Skip empty lines and delimiter lines until the first header line.
        loop {
            let Some((line, next)) = read_line(&self.buffer, cursor) else {
                return self.need_more();
            };

            if line.is_empty() {
                cursor = next;
                continue;
            }

            if contains(line, DEVICE_BOUNDARY.as_bytes()) {
                // The closing delimiter has two extra dashes after the boundary.
                if line.ends_with(b"--")
                    && line.len() >= DEVICE_BOUNDARY.len() + 2
                    && contains(&line[..line.len() - 2], DEVICE_BOUNDARY.as_bytes())
                {
                    self.buffer.drain(..next);
                    return Ok(Some(Frame::End));
                }
                cursor = next;
                continue;
            }

            break;
        }

        let mut headers = Vec::new();
        loop {
            let Some((line, next)) = read_line(&self.buffer, cursor) else {
                return self.need_more();
            };
            cursor = next;

            if line.is_empty() {
                break;
            }

            let line =
                std::str::from_utf8(line).context("media stream part header is not UTF-8")?;
            // Lines without a colon are ignored, like the Tapo app does.
            if let Some((name, value)) = line.split_once(':') {
                headers.push((name.trim().to_ascii_lowercase(), value.trim().to_string()));
            }
        }

        let content_length = headers
            .iter()
            .find(|(name, _)| name == "content-length")
            .and_then(|(_, value)| value.parse::<usize>().ok())
            .context("media stream part is missing a valid Content-Length")?;
        if content_length > MAX_BODY_SIZE {
            bail!("media stream part body of {content_length} bytes exceeds {MAX_BODY_SIZE} bytes");
        }

        let body_end = cursor + content_length;
        if self.buffer.len() < body_end {
            return self.need_more();
        }

        let body = self.buffer[cursor..body_end].to_vec();

        // Consume the trailing line ending when it has already arrived, so
        // that a fully received part leaves nothing behind.
        let rest = &self.buffer[body_end..];
        let trailer = if rest.starts_with(b"\r\n") {
            2
        } else if rest.starts_with(b"\n") {
            1
        } else {
            0
        };
        self.buffer.drain(..body_end + trailer);

        Ok(Some(Frame::Part(Part { headers, body })))
    }

    fn need_more(&self) -> anyhow::Result<Option<Frame>> {
        if self.buffer.len() > MAX_BUFFER_SIZE {
            bail!(
                "no complete media stream part in the first {} buffered bytes",
                self.buffer.len()
            );
        }
        Ok(None)
    }
}

/// Returns the line that starts at `start` (without its line ending) and the
/// offset of the next line. Accepts both `\r\n` and bare `\n`, like the
/// `DataInputStream::readLine` the Tapo app uses.
fn read_line(buffer: &[u8], start: usize) -> Option<(&[u8], usize)> {
    let newline = buffer[start..].iter().position(|byte| *byte == b'\n')?;
    let mut line = &buffer[start..start + newline];
    if let Some(stripped) = line.strip_suffix(b"\r") {
        line = stripped;
    }
    Some((line, start + newline + 1))
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn device_part(headers: &str, body: &[u8]) -> Vec<u8> {
        let mut part = Vec::new();
        part.extend_from_slice(b"--device-stream-boundary--\r\n");
        part.extend_from_slice(headers.as_bytes());
        part.extend_from_slice(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes());
        part.extend_from_slice(body);
        part.extend_from_slice(b"\r\n");
        part
    }

    #[test]
    fn test_encode_client_part() {
        let part = encode_client_part(
            &[
                ("X-Data-Window-Size", "50"),
                ("Content-Type", "application/json"),
            ],
            b"{\"a\":1}",
        );

        assert_eq!(
            part,
            b"----client-stream-boundary--\r\n\
              X-Data-Window-Size: 50\r\n\
              Content-Type: application/json\r\n\
              Content-Length: 7\r\n\
              \r\n\
              {\"a\":1}\r\n"
        );
    }

    #[test]
    fn test_parse_consecutive_parts() {
        let mut bytes = device_part(
            "Content-Type: application/json\r\nX-Session-Id: 7\r\n",
            b"{\"type\":\"response\"}",
        );
        bytes.extend(device_part(
            "Content-Type: video/mp2t\r\nX-Data-Sequence: 1\r\n",
            &[0x47; 188],
        ));

        let mut parser = PartParser::new(bytes);

        let Some(Frame::Part(json)) = parser.next_frame().unwrap() else {
            panic!("expected a JSON part");
        };
        assert!(json.is_json());
        assert_eq!(json.header("x-session-id"), Some("7"));
        assert_eq!(json.body, b"{\"type\":\"response\"}");

        let Some(Frame::Part(media)) = parser.next_frame().unwrap() else {
            panic!("expected a media part");
        };
        assert_eq!(media.content_type(), Some("video/mp2t"));
        assert_eq!(media.header("x-data-sequence"), Some("1"));
        assert_eq!(media.body.len(), 188);
        assert!(!media.is_json());

        assert_eq!(parser.next_frame().unwrap(), None);
        assert!(parser.buffer_mut().is_empty());
    }

    #[test]
    fn test_parse_waits_for_complete_part() {
        let bytes = device_part("Content-Type: video/mp2t\r\n", b"0123456789");
        let mut parser = PartParser::new(Vec::new());

        for (index, byte) in bytes.iter().enumerate() {
            parser.buffer_mut().push(*byte);
            let frame = parser.next_frame().unwrap();
            // The body's last byte is followed by the trailing CRLF, which
            // is not needed to complete the part.
            if index + 1 < bytes.len() - 2 {
                assert_eq!(frame, None, "frame completed early at byte {index}");
            } else if index + 1 == bytes.len() - 2 {
                let Some(Frame::Part(part)) = frame else {
                    panic!("expected the part at byte {index}");
                };
                assert_eq!(part.body, b"0123456789");
            } else {
                assert_eq!(frame, None);
            }
        }
    }

    #[test]
    fn test_parse_skips_empty_lines_and_ignores_lines_without_colon() {
        let mut parser = PartParser::new(
            b"\r\n\r\n--device-stream-boundary--\r\nnot a header\r\nContent-Length: 2\r\n\r\nok\r\n"
                .to_vec(),
        );

        let Some(Frame::Part(part)) = parser.next_frame().unwrap() else {
            panic!("expected a part");
        };
        assert_eq!(
            part.headers,
            vec![("content-length".to_string(), "2".to_string())]
        );
        assert_eq!(part.body, b"ok");
    }

    #[test]
    fn test_parse_accepts_bare_newlines() {
        let mut parser =
            PartParser::new(b"--device-stream-boundary--\nContent-Length: 3\n\nabc\n".to_vec());

        let Some(Frame::Part(part)) = parser.next_frame().unwrap() else {
            panic!("expected a part");
        };
        assert_eq!(part.body, b"abc");
    }

    #[test]
    fn test_parse_closing_delimiter() {
        let mut parser = PartParser::new(b"\r\n--device-stream-boundary----\r\n".to_vec());
        assert_eq!(parser.next_frame().unwrap(), Some(Frame::End));
    }

    #[test]
    fn test_parse_rejects_missing_content_length() {
        let mut parser = PartParser::new(
            b"--device-stream-boundary--\r\nContent-Type: video/mp2t\r\n\r\nabc\r\n".to_vec(),
        );
        assert!(parser.next_frame().is_err());
    }

    #[test]
    fn test_parse_rejects_oversized_body() {
        let mut parser = PartParser::new(
            format!(
                "--device-stream-boundary--\r\nContent-Length: {}\r\n\r\n",
                MAX_BODY_SIZE + 1
            )
            .into_bytes(),
        );
        assert!(parser.next_frame().is_err());
    }
}
