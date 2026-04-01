use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
// ===========================================

pub fn decode_base64_args_fun(encoded_args: &str) -> Option<String> {
    BASE64
        .decode(encoded_args)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
}
// ===========================================
