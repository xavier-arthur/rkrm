pub trait IntoString {
    fn into_string(self) -> String;
}

impl IntoString for Vec<u8> {
    fn into_string(self) -> String {
        String::from_utf8(self).unwrap_or("".to_string())
    }
}