pub fn strtok<'a>(s: &'_ mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // fn check_is_static(_: &'static str) {}

        let mut x = "Hello world";
        // check_is_static(x);
        let hello = strtok(&mut x, ' ');
        assert_eq!(hello, "Hello");
        assert_eq!(x, "world");
    }
}
