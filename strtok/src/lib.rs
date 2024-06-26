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

#[derive(Debug)]
struct Delimiter<'a> {
    index: usize,
    delimter: &'a str,
}

pub fn strtokn<'a>(s: &'_ mut &'a str, delimters: &'_ [&'_ str]) -> &'a str {
    let (i, l) = loop {
        let mut dmt: Option<Delimiter> = None;
        for &d in delimters {
            if let Some(i) = s.find(d) {
                dmt = match dmt {
                    Some(dmt) => Some(Delimiter {
                        index: std::cmp::min(dmt.index, i),
                        delimter: if i < dmt.index { d } else { dmt.delimter },
                    }),
                    _ => Some(Delimiter {
                        index: i,
                        delimter: d,
                    }),
                };
            }
        }

        // println!("{:?}", dmt);
        match dmt {
            Some(d) => {
                if d.index == 0 {
                    *s = &s[d.delimter.len()..];
                    continue;
                } else {
                    break (d.index, d.delimter.len());
                }
            }
            _ => break (s.len(), 0),
        }
    };

    let prefix = &s[..i];
    let suffix = &s[(i + l)..];
    *s = suffix;
    prefix
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

    #[test]
    fn test_strtokn() {
        let mut x = "- This, a sample string.";
        let delimters = [" ", ",", ".", "-"];

        let mut tok = strtokn(&mut x, &delimters);
        assert_eq!(tok, "This");
        assert_eq!(x, " a sample string.");

        tok = strtokn(&mut x, &delimters);
        assert_eq!(tok, "a");
        assert_eq!(x, "sample string.");

        tok = strtokn(&mut x, &delimters);
        assert_eq!(tok, "sample");
        assert_eq!(x, "string.");

        tok = strtokn(&mut x, &delimters);
        assert_eq!(tok, "string");
        assert_eq!(x, "");

        tok = strtokn(&mut x, &delimters);
        assert_eq!(tok, "");
        assert_eq!(x, "");
    }
}
