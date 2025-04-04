use crate::*;

/// A single space character.
///
/// This constant is used to represent a space character in string
/// or byte operations.
pub static SPACE: &str = " ";

/// The byte representation of a single space character.
///
/// This constant provides the byte equivalent of the space character
/// for use in low-level operations.
pub static SPACE_U8: u8 = SPACE.as_bytes()[0];

/// A tab character.
///
/// This constant is used to represent a tab character in string
/// or byte operations.
pub static TAB: &str = "\t";

/// The byte representation of a tab character.
///
/// This constant provides the byte equivalent of the tab character
/// for use in low-level operations.
pub static TAB_U8: u8 = TAB.as_bytes()[0];

/// A line break character (newline).
///
/// This constant is used to represent a line break character in
/// string or byte operations.
pub static BR: &str = "\n";

/// A static byte slice representation of the string `BR`.
pub static BR_BYTES: &[u8] = BR.as_bytes();

/// A colon followed by a space (`: `).
///
/// This constant is commonly used in formatted strings, such as
/// headers or key-value pairs, where a colon and a space are needed.
pub static COLON_SPACE: &str = ": ";

/// The byte representation of the first character in the `COLON_SPACE`.
///
/// This constant provides the byte equivalent of the colon character
/// from the `COLON_SPACE` string.
pub static COLON_SPACE_BYTES: &[u8] = COLON_SPACE.as_bytes();

/// A colon followed by a space symbol (`:`).
///
/// This constant is commonly used in formatted strings, such as
/// headers or key-value pairs, where a colon and a space are needed.
pub static COLON_SPACE_SYMBOL: &str = ":";

/// Query symbols
pub static QUERY_SYMBOL: &str = "?";

/// Hash symbols
pub static HASH_SYMBOL: &str = "#";

/// Empty str
pub static EMPTY_STR: &str = "";

/// Default host
pub static DEFAULT_HOST: &str = "0.0.0.0";

/// Default web port
pub static DEFAULT_WEB_PORT: usize = 80;

/// Http br
pub static HTTP_BR: &str = "\r\n";

/// Http br bytes
pub static HTTP_BR_BYTES: &[u8] = HTTP_BR.as_bytes();

/// Http double br
pub static HTTP_DOUBLE_BR: &str = "\r\n\r\n";

/// Http double br bytes
pub static HTTP_DOUBLE_BR_BYTES: &[u8] = HTTP_DOUBLE_BR.as_bytes();

/// Default http path
pub static DEFAULT_HTTP_PATH: &str = "/";

/// Default http path bytes
pub static DEFAULT_HTTP_PATH_BYTES: &[u8] = DEFAULT_HTTP_PATH.as_bytes();

/// And
pub static AND: &str = "&";

/// And bytes
pub static AND_BYTES: &[u8] = AND.as_bytes();

/// Equal
pub static EQUAL: &str = "=";

/// Equal bytes
pub static EQUAL_BYTES: &[u8] = EQUAL.as_bytes();

/// Zero str
pub static ZERO_STR: &str = "0";

/// Zero str bytes
pub static ZERO_STR_BYTES: &[u8] = ZERO_STR.as_bytes();

/// Default buffer size
pub static DEFAULT_BUFFER_SIZE: usize = 4096;

/// Default max redirect times
pub static DEFAULT_MAX_REDIRECT_TIMES: usize = 8;

/// Default timeout
pub const DEFAULT_TIMEOUT: u64 = u64::MAX;

/// Point
pub const POINT: &str = ".";

/// Root path
pub const ROOT_PATH: &str = "/";

/// Semicolon
pub const SEMICOLON: &str = ";";

/// Semicolon and space
pub const SEMICOLON_SPACE: &str = "; ";

/// OK
pub static OK: &str = "OK";

/// GUID
pub static GUID: &[u8; 36] = b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

/// HASH_STATE
pub static HASH_STATE: [u32; 5] = [
    0x67452301u32,
    0xEFCDAB89,
    0x98BADCFE,
    0x10325476,
    0xC3D2E1F0,
];

/// BASE64_CHARSET_TABLE
pub static BASE64_CHARSET_TABLE: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// MAX_FRAME_SIZE
pub static MAX_FRAME_SIZE: usize = 65535;

/// DEFAULT_SOCKET_ADDR
pub static DEFAULT_SOCKET_ADDR: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0));

/// SOCKET_ADDR_127_0_0_1
pub static SOCKET_ADDR_127_0_0_1: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0));

/// hyperlane
pub static HYPERLANE: &str = "hyperlane";

/// Hyperlane
pub static HYPERLANE_PASCAL_CASE: &str = "Hyperlane";

/// DEFAULT_INNER_PRINT
pub static DEFAULT_INNER_PRINT: bool = true;

/// DEFAULT_NODELAY
pub static DEFAULT_INNER_LOG: bool = true;

/// DEFAULT_NODELAY
pub static DEFAULT_NODELAY: bool = false;

/// DEFAULT_LINGER
pub static DEFAULT_LINGER: Option<Duration> = None;

/// DEFAULT_TTI
pub static DEFAULT_TTI: Option<u32> = None;
