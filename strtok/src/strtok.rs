pub fn strtok<'a>(s: &'a mut &str, pat: char) -> &'a str {
    match s.find(pat) {
        Some(i) => {
            let prefix = &s[..i];
            let suffix = &s[i + pat.len_utf8()..];
            *s = suffix;
            prefix
        }
        None => {
            let prefix = *s;
            *s = "";
            prefix
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = "hello world";

        let s1 = &mut s;
        let token_1 = strtok(s1, ' ');

        assert_eq!(token_1, "hello");
        assert_eq!(*s1, "world");
    }
}
