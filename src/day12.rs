use regex_macro::regex;

pub type Num = u32;
pub type Fountains = Vec<Fountain>;
pub type Groups = Vec<u8>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Fountain {
    Good,
    Broken,
    Unknown,
}

impl Fountain {
    fn parse(s: &str) -> Option<Self> {
        assert!(s.len() == 1);
        match s {
            r"." => Some(Self::Good),
            r"#" => Some(Self::Broken),
            r"?" => Some(Self::Unknown),
            _ => None,
        }
    }
}

pub fn parse_fountain_line(line: &str) -> (Vec<Fountain>, Vec<u8>) {
    let re_fountains = regex!(r"[.#?]");
    let fountains = re_fountains
        .find_iter(&line)
        .map(|m| Fountain::parse(m.as_str()).unwrap())
        .collect();

    let re_digits = regex!(r"[0-9]+");
    let cluster_sizes = re_digits
        .find_iter(&line)
        .map(|m| m.as_str().parse::<u8>().unwrap())
        .collect();

    let result: (Vec<Fountain>, Vec<u8>) = (fountains, cluster_sizes);
    result
}

#[cfg(test)]
mod test {
    use super::Fountain::*;
    use super::*;

    #[test]
    fn test_parse_fountain_char() {
        let expected = [Some(Unknown), Some(Broken), Some(Good), None];
        let actual = [
            Fountain::parse("?"),
            Fountain::parse("#"),
            Fountain::parse("."),
            Fountain::parse("X"),
        ];
        assert_eq!(&actual, &expected)
    }

    #[test]
    fn test_parse_fountain_line() {
        let input = r"??#?#. 1,1,7";
        let expected: (Vec<Fountain>, Vec<u8>) = (
            vec![Unknown, Unknown, Broken, Unknown, Broken, Good],
            vec![1, 1, 7],
        );
        let actual: (Vec<Fountain>, Vec<u8>) = parse_fountain_line(&input);
        assert_eq!(&actual, &expected)
    }
}
