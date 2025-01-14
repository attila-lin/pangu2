//! Paranoid text spacing for good readability, to automatically insert
//! whitespace between CJK (Chinese, Japanese, Korean) and half-width
//! characters (alphabetical letters, numerical digits and symbols).

use std::borrow::Cow;

use const_format::formatcp;
use lazy_regex::lazy_regex;
use once_cell::sync::Lazy;
use regex::Regex;

// Chinese, Japanese, Korean
// See <http://unicode-table.com>
const CJK: &str = "\u{2e80}-\u{2eff}\u{2f00}-\u{2fdf}\u{3040}-\u{309f}\u{30a0}-\u{30ff}\u{3100}-\u{312f}\u{3200}-\u{32ff}\u{3400}-\u{4dbf}\u{4e00}-\u{9fff}\u{f900}-\u{faff}";

// Alphabets, Numbers, Symbols
const ANS: &str =
    "A-Za-z0-9`\\$%\\^&\\*\\-=\\+\\\\|/\u{00a1}-\u{00ff}\u{2022}\u{2027}\u{2150}-\u{218f}";

/// Insert whitespace between CJK and half-width characters.
pub fn spacing(text: &str) -> Cow<str> {
    // <https://github.com/vinta/pangu/blob/master/pangu.go>
    static CJK_QUOTE: Lazy<Regex> =
        Lazy::new(|| Regex::new(formatcp!("([{CJK}])([\"'])", CJK = CJK)).unwrap());
    static QUOTE_CJK: Lazy<Regex> =
        Lazy::new(|| Regex::new(formatcp!("([\"'])([{CJK}])", CJK = CJK)).unwrap());
    static FIX_QUOTE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(formatcp!(
            "([\"'\\(\\[\\{{<\u{201c}])(\\s*)(.+?)(\\s*)([\"'\\)\\]\\}}>\u{201d}])"
        ))
        .unwrap()
    });
    static FIX_SINGLE_QUOTE: Lazy<Regex> =
        Lazy::new(|| Regex::new(formatcp!("([{CJK}])( )(')([A-Za-z])", CJK = CJK)).unwrap());
    // -----
    static CJK_HASH: Lazy<Regex> =
        Lazy::new(|| Regex::new(formatcp!("([{CJK}])(#(\\S+))", CJK = CJK)).unwrap());
    static HASH_CJK: Lazy<Regex> =
        Lazy::new(|| Regex::new(formatcp!("((\\S+)#)([{CJK}])", CJK = CJK)).unwrap());
    // -----
    static CJK_OPERATOR_ANS: Lazy<Regex> = Lazy::new(|| {
        Regex::new(formatcp!(
            "([{CJK}])([\\+\\-\\*/=&\\|<>])([A-Za-z0-9])",
            CJK = CJK
        ))
        .unwrap()
    });
    static ANS_OPERATOR_CJK: Lazy<Regex> = Lazy::new(|| {
        Regex::new(formatcp!(
            "([A-Za-z0-9])([\\+\\-\\*/=&\\|<>])([{CJK}])",
            CJK = CJK
        ))
        .unwrap()
    });
    // -----
    static CJK_BRACKET_CJK: Lazy<Regex> = Lazy::new(|| {
        Regex::new(formatcp!(
            "([{CJK}])([\\(\\[\\{{<\u{201c}]+(.*?)[\\)\\]\\}}>\u{201d}]+)([{CJK}])",
            CJK = CJK
        ))
        .unwrap()
    });
    static CJK_BRACKET: Lazy<Regex> = Lazy::new(|| {
        Regex::new(formatcp!("([{CJK}])([\\(\\[\\{{<\u{201c}>])", CJK = CJK)).unwrap()
    });
    static BRACKET_CJK: Lazy<Regex> = Lazy::new(|| {
        Regex::new(formatcp!("([\\)\\]\\}}>\u{201d}<])([{CJK}])", CJK = CJK)).unwrap()
    });
    static FIX_BRACKET: Lazy<Regex> =
        lazy_regex!("([\\(\\[\\{{<\u{201c}]+)(\\s*)(.+?)(\\s*)([\\)\\]\\}}>\u{201d}]+)");
    // -----
    static FIX_SYMBOL: Lazy<Regex> = Lazy::new(|| {
        Regex::new(formatcp!(
            "([{CJK}])([~!;:,\\.\\?\u{2026}])([A-Za-z0-9])",
            CJK = CJK
        ))
        .unwrap()
    });
    // -----
    static CJK_ANS: Lazy<Regex> =
        Lazy::new(|| Regex::new(formatcp!("([{CJK}])([{ANS}@])", CJK = CJK, ANS = ANS)).unwrap());
    static ANS_CJK: Lazy<Regex> = Lazy::new(|| {
        Regex::new(formatcp!(
            "([{ANS}~!;:,\\.\\?\u{2026}])([{CJK}])",
            CJK = CJK,
            ANS = ANS
        ))
        .unwrap()
    });

    let text_1 = CJK_QUOTE.replace_all(text, "$1 $2");
    let text_2 = QUOTE_CJK.replace_all(&text_1, "$1 $2");
    let text_3 = FIX_QUOTE.replace_all(&text_2, "$1$3$5");
    let text_4 = FIX_SINGLE_QUOTE.replace_all(&text_3, "$1$3$4");

    let text_5 = CJK_HASH.replace_all(&text_4, "$1 $2");
    let text_6 = HASH_CJK.replace_all(&text_5, "$1 $3");

    let text_7 = CJK_OPERATOR_ANS.replace_all(&text_6, "$1 $2 $3");
    let text_8 = ANS_OPERATOR_CJK.replace_all(&text_7, "$1 $2 $3");

    let old_text = text_8;
    let new_text = CJK_BRACKET_CJK.replace_all(&old_text, "$1 $2 $4");
    let text_ir;
    let text_11 = if *new_text == *old_text {
        text_ir = CJK_BRACKET.replace_all(&new_text, "$1 $2");
        BRACKET_CJK.replace_all(&text_ir, "$1 $2")
    } else {
        new_text
    };
    let text_12 = FIX_BRACKET.replace_all(&text_11, "$1$3$5");

    let text_13 = FIX_SYMBOL.replace_all(&text_12, "$1$2 $3");

    let text_14 = CJK_ANS.replace_all(&text_13, "$1 $2");
    let text_15 = ANS_CJK.replace_all(&text_14, "$1 $2");

    let eq = |x: &str, y: &str| x.as_ptr() == y.as_ptr() && x.len() == y.len();

    match text_15 {
        Cow::Borrowed(t) if eq(t, text) => Cow::Borrowed(text),
        spaced => Cow::Owned(spaced.into_owned()),
    }
}
