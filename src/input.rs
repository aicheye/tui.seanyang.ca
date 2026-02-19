/// Parsed key events from raw SSH byte input (VT100/ANSI).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Enter,
    Backspace,
    Escape,
    Tab,
    Home,
    End,
    PageUp,
    PageDown,
    // Vim-style aliases handled at the app layer
}

/// Parse raw bytes received over SSH into a sequence of key events.
/// Handles multi-byte ANSI escape sequences.
pub fn parse_keys(bytes: &[u8]) -> Vec<Key> {
    let mut keys = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        let key = match bytes[i] {
            // Enter
            b'\r' | b'\n' => {
                i += 1;
                Some(Key::Enter)
            }
            // Backspace / DEL
            b'\x7f' | b'\x08' => {
                i += 1;
                Some(Key::Backspace)
            }
            // Tab
            b'\t' => {
                i += 1;
                Some(Key::Tab)
            }
            // Escape sequence or bare Escape
            b'\x1b' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'[' {
                    // CSI sequence: ESC [ ...
                    let seq_start = i + 2;
                    let mut seq_end = seq_start;
                    while seq_end < bytes.len()
                        && !matches!(bytes[seq_end], b'A'..=b'Z' | b'a'..=b'z' | b'~')
                    {
                        seq_end += 1;
                    }

                    let terminator = if seq_end < bytes.len() {
                        seq_end += 1;
                        Some(bytes[seq_end - 1])
                    } else {
                        None
                    };

                    let param = &bytes[seq_start..seq_end.saturating_sub(1)];
                    let parsed = match (param, terminator) {
                        (b"", Some(b'A')) => Some(Key::Up),
                        (b"", Some(b'B')) => Some(Key::Down),
                        (b"", Some(b'C')) => Some(Key::Right),
                        (b"", Some(b'D')) => Some(Key::Left),
                        (b"", Some(b'H')) | (b"1", Some(b'~')) => Some(Key::Home),
                        (b"", Some(b'F')) | (b"4", Some(b'~')) => Some(Key::End),
                        (b"5", Some(b'~')) => Some(Key::PageUp),
                        (b"6", Some(b'~')) => Some(Key::PageDown),
                        _ => None,
                    };
                    i = seq_end;
                    parsed
                } else {
                    // Bare ESC
                    i += 1;
                    Some(Key::Escape)
                }
            }
            // Printable ASCII
            c if c.is_ascii_graphic() || c == b' ' => {
                i += 1;
                Some(Key::Char(c as char))
            }
            // Unknown / control byte — skip
            _ => {
                i += 1;
                None
            }
        };

        if let Some(k) = key {
            keys.push(k);
        }
    }

    keys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_chars() {
        assert_eq!(
            parse_keys(b"hello"),
            vec![
                Key::Char('h'),
                Key::Char('e'),
                Key::Char('l'),
                Key::Char('l'),
                Key::Char('o'),
            ]
        );
    }

    #[test]
    fn test_enter_and_escape() {
        assert_eq!(parse_keys(b"\r"), vec![Key::Enter]);
        assert_eq!(parse_keys(b"\x1b"), vec![Key::Escape]);
        assert_eq!(parse_keys(b"\x7f"), vec![Key::Backspace]);
    }
}
