#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use textwrap::wrap_algorithms::wrap_optimal_fit;
use textwrap::{core, wrap_algorithms};

#[derive(Arbitrary, Debug)]
struct Penalties {
    nline_penalty: usize,
    overflow_penalty: usize,
    short_last_line_fraction: usize,
    short_last_line_penalty: usize,
    hyphen_penalty: usize,
}

impl Into<wrap_algorithms::Penalties> for Penalties {
    fn into(self) -> wrap_algorithms::Penalties {
        wrap_algorithms::Penalties {
            nline_penalty: self.nline_penalty,
            overflow_penalty: self.overflow_penalty,
            short_last_line_fraction: std::cmp::max(1, self.short_last_line_fraction),
            short_last_line_penalty: self.short_last_line_penalty,
            hyphen_penalty: self.hyphen_penalty,
        }
    }
}

#[derive(Arbitrary, Debug, PartialEq)]
struct Word {
    width: usize,
    whitespace_width: usize,
    penalty_width: usize,
}

#[rustfmt::skip]
impl core::Fragment for Word {
    fn width(&self) -> f64 { self.width as f64 }
    fn whitespace_width(&self) -> f64 { self.whitespace_width as f64 }
    fn penalty_width(&self) -> f64 { self.penalty_width as f64 }
}

// Check wrapping fragments generated with integer widths. These
// fragments are of the same form as the ones generated by wrap.
fuzz_target!(|input: (usize, Vec<Word>, Penalties)| {
    let width = input.0;
    let words = input.1;
    let penalties = input.2.into();
    let _ = wrap_optimal_fit(&words, &[width as f64], &penalties);
});
