#![doc = include_str!("../README.md")]

use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
mod tests;

/**
Wrap Unicode text with ANSI color codes
*/
pub fn termwrap(s: &str, width: usize, continuation: &str) -> String {
    if width == 0 {
        return s.to_string();
    }

    // Input without ANSI color codes to graphemes
    let b = String::from_utf8(strip_ansi_escapes::strip(s.as_bytes()).unwrap()).unwrap();
    let mut gb = b.graphemes(true).collect::<Vec<_>>();

    // Input graphemes
    let mut ga = s.graphemes(true).collect::<Vec<_>>();

    // Continuation graphemes
    let gc = continuation.graphemes(true).collect::<Vec<_>>();
    let cw = gc.len();
    let cwp = cw + 1;
    let w = width - cw; // initial max width; leave space for the continuation

    let mut r = String::new(); // result
    let mut l = 0; // current column
    let mut ca; // current grapheme from `ga`
    let mut cb; // current grapheme from `gb`

    // Internal function update state
    fn update(ca: &str, r: &mut String, l: &mut usize, counts: bool) {
        if ca == "\t" {
            // Tab counts as a color code for some reason, so it always counts
            // against the line length.
            // `8 - *l % 8` is the number of spaces from the current column to
            // the nearest multiple of 8.
            for _ in 0..(8 - *l % 8) {
                *l += 1;
                r.push(' ');
            }
        } else {
            // Not a tab, so it only counts if not a color code.
            if counts {
                *l = if ca == "\n" { 0 } else { *l + 1 };
            }
            r.push_str(ca);
        }
    }

    while !ga.is_empty() {
        // Process line until the initial max width
        while l < w {
            if ga.is_empty() {
                break;
            }
            ca = ga.remove(0);
            cb = gb.remove(0);

            // Process extra ANSI color code graphemes
            while ca != cb && !ga.is_empty() {
                update(ca, &mut r, &mut l, false);
                ca = ga.remove(0);
            }

            // Process regular graphemes
            update(ca, &mut r, &mut l, true);
        }

        // Process the last character(s) of the line
        let gal = ga.len();
        if gal == cw || (gal == cwp && ga[1] == "\n") {
            // Continuation width characters left in the input or the current line, so just push
            // them
            r.push_str(ga.remove(0));
        } else if gal != 0 {
            // Unless done, insert the continuation
            r.push_str(continuation);
            r.push('\n');
        }
        l = 0;
    }

    r
}
