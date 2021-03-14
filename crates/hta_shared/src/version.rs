use std::str::FromStr;

pub fn is_version_str_ge(v1_str: &str, v2_str: &str) -> Option<bool> {
    let v1 = parse_version_str(v1_str);
    let v2 = parse_version_str(v2_str);

    match (v1, v2) {
        (Some(tup1), Some(tup2)) => Some(is_version_ge(tup1, tup2)),
        _ => None,
    }
}

//TODO This function can be improved.
pub fn is_version_ge(v1: (u64, u64, u64), v2: (u64, u64, u64)) -> bool {
    if v1.0 > v2.0 {
        return true;
    }
    else if v1.0 == v2.0 {
        if v1.1 > v2.1 {
            return true;
        }
        else if v1.1 == v2.1 {
            return v1.2 > v2.2 || v1.2 == v2.2;
        }
    }
    false
}

pub fn parse_version_str(v_str: &str) -> Option<(u64, u64, u64)> {
    let sv: Vec<&str> = v_str.split(".").collect();
    if sv.len() == 3 {
        let major = u64::from_str(sv.get(0).unwrap());
        let minor = u64::from_str(sv.get(1).unwrap());
        let patch = u64::from_str(sv.get(2).unwrap());

        match (major, minor, patch) {
            (Ok(maj), Ok(min), Ok(pat)) => return Some((maj, min, pat)),
            _ => {}
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(parse_version_str(""), None);
        assert_eq!(parse_version_str("-1.0.0"), None);
        assert_eq!(parse_version_str("1.674...3634"), None);
        assert_eq!(parse_version_str("1.1"), None);

        assert_eq!(parse_version_str("1.1.0"), Some((1, 1, 0)));
        assert_eq!(parse_version_str("1.1.3214"), Some((1, 1, 3214)));
        assert_eq!(
            parse_version_str("723498.1.124523"),
            Some((723498, 1, 124523))
        );
        assert_eq!(parse_version_str("0.0.0"), Some((0, 0, 0)));
    }

    #[test]
    fn version_ge() {
        assert_eq!(is_version_ge((0, 0, 2), (0, 0, 3)), false);
        assert_eq!(is_version_ge((1, 0, 2), (1, 0, 3)), false);
        assert_eq!(is_version_ge((1, 0, 0), (1, 5, 3)), false);

        assert_eq!(is_version_ge((1, 0, 2), (0, 0, 3)), true);
        assert_eq!(is_version_ge((0, 0, 255), (0, 0, 3)), true);
        assert_eq!(is_version_ge((0, 5, 25), (0, 0, 3)), true);
    }

    #[test]
    fn version_str_ge() {
        assert_eq!(is_version_str_ge("", ""), None);
        assert_eq!(is_version_str_ge("-1.0.0", "1.0.0"), None);
        assert_eq!(is_version_str_ge("1.0.0", "-1.0.0"), None);

        assert_eq!(is_version_str_ge("1.0.0", "1.0.1"), Some(false));
        assert_eq!(is_version_str_ge("1.0.0", "1.2.0"), Some(false));

        assert_eq!(is_version_str_ge("25.0.0", "1.0.0"), Some(true));
        assert_eq!(is_version_str_ge("1.5.0", "0.2.0"), Some(true));
    }
}
