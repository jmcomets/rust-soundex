extern crate itertools;

use itertools::Itertools;
use std::ascii::AsciiExt;

fn american_soundex_code(c: u8) -> Option<u8> {
    match c {
        b'B' | b'F' | b'P' | b'V'                             => Some(b'1'),
        b'C' | b'G' | b'J' | b'K' | b'Q' | b'S' | b'X' | b'Z' => Some(b'2'),
        b'D' | b'T'                                           => Some(b'3'),
        b'L'                                                  => Some(b'4'),
        b'M' | b'N'                                           => Some(b'5'),
        b'R'                                                  => Some(b'6'),
        _                                                     => None,
    }
}

/// Performs Soundex calculation on a string passed in
///
/// # Examples
///
/// ```
/// use soundex;
/// let code: String = soundex::american_soundex("Sirname");
/// assert_eq!(&code, "S655");
/// ```
pub fn american_soundex(s: &str) -> String {
    let chars = move || {
        s.chars()
            .filter(|c| c.is_ascii())
            .filter_map(|c| c.to_uppercase().next())
            .map(|c| c as u8)
    };

    let mut codes: Vec<_> = chars()

        // remove H and W from the tail
        .take(1).chain(chars().skip(1).filter(|&c| c != b'H' && c != b'W'))

        // get the codes for each character (or None)
        .map(|c| (c, american_soundex_code(c)))

        // remove adjacent chars
        .group_by(|&(_, code)| code).into_iter()
            .filter_map(|(_, mut g)| g.next())

        // remove entries without codes (except first character)
        .enumerate().filter_map(|(i, (c, code))| {
            if i == 0 {
                Some(c)
            } else {
                code
            }
        })

        // pad with trailing zeros
        .pad_using(4, |_| b'0').take(4)
        .collect();

    // if the first code is a digit, replace with the first character
    if b'1' <= codes[0] && codes[0] <= b'9' {
        codes[0] = chars().next().unwrap();
    }

    // unsafe is ok here since we've already checked that the string is ascii
    debug_assert!(codes.iter().all(|c| c.is_ascii()));
    unsafe { String::from_utf8_unchecked(codes) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn american_soundex_correct() {
        let params: Vec<(&str, &str)> = vec![
            ("",           "0000"),
            ("007bond",    "0153"),
            ("Ashcraft",   "A261"),
            ("Pfister",    "P236"),
            ("Robert",     "R163"),
            ("Rubin",      "R150"),
            ("Rupert",     "R163"),
            ("Toto",       "T300"),
            ("Tymczak",    "T522"),
            ("husobee",    "H210"),
            ("touchstone", "T235"),
            ("heart ❤", "H630"),
        ];

        for (i, o) in params {
            assert_eq!(american_soundex(i), o.to_string());
        }
    }

}
