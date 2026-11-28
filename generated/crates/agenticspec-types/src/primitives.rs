// generated from agenticspec v1
// contract digest ccb8808b577e000f62b25eea26b56257870a4af7bcbda0f276d5b2bb26a8a862
// do not edit: regenerate with `ess synthesize`

//! How the specification's primitives are spelled in this workspace.
//!
//! Four map onto types that already mean exactly the same thing: `String` stays `String`,
//! `Boolean` is `bool`, `Integer` is `i64`, `Bytes` is `Vec<u8>`. The four below have no `std`
//! equivalent, and no dependency is taken for them — this workspace builds from exactly its
//! committed bytes. Each is a transparent wrapper over its wire rendering, distinct from `String`
//! and from each other for the same reason the specification's own newtypes are distinct from
//! their representations: a value's meaning is not its shape.

/// An exact decimal, carried as its wire rendering — a decimal string such as `10.50`.
///
/// Never a float: money does not round the way a float does. Equality and order are over the
/// rendering, so `1.5` and `1.50` are different values here; arithmetic is deliberately absent,
/// because the specification declares no operation on a decimal, so there is none to generate.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Decimal(pub String);

/// An instant, carried as its wire rendering — RFC 3339, such as `2026-01-01T00:00:00Z`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(pub String);

/// A length of time, carried as its wire rendering — an ISO 8601 duration such as `P30D`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration(pub String);

/// A UUID, carried as its canonical textual rendering.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uuid(pub String);

/// A finite IEEE 754 binary64, preserving the sign of zero.
///
/// The field is private, so a NaN or an infinity cannot be held: [`Binary64::new`] refuses
/// both. Equality is IEEE 754's, so `+0` and `-0` are equal, and `Eq` is lawful because no NaN
/// can be held. There is no `Hash` and no `Ord`: guards and orders compare the decimal
/// `Display` writes. The sign of zero stays readable through [`Binary64::get`],
/// [`Binary64::token`] and `Display`.
#[derive(Debug, Clone, Copy)]
pub struct Binary64(f64);

/// Why a value is not a [`Binary64`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Binary64Error {
    /// A NaN or an infinity.
    NonFinite,
}

impl std::fmt::Display for Binary64Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Binary64 requires a finite value")
    }
}

impl std::error::Error for Binary64Error {}

impl Binary64 {
    /// The value, when it is finite.
    ///
    /// # Errors
    ///
    /// [`Binary64Error::NonFinite`] for a NaN or an infinity.
    pub fn new(value: f64) -> Result<Self, Binary64Error> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(Binary64Error::NonFinite)
        }
    }

    /// The wrapped value.
    #[must_use]
    pub fn get(self) -> f64 {
        self.0
    }

    /// A JSON number token, rounded once to the nearest binary64; `None` where it rounds to
    /// an infinity or spells no number.
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        token.parse::<f64>().ok().and_then(|value| Self::new(value).ok())
    }

    /// The JSON number the wire writes: the shortest digits that read back as the value,
    /// laid out as `3.0`, `-0.0`, `0.73`, `1e16` and `1e-6` are.
    #[must_use]
    pub fn token(&self) -> String {
        let value = self.0;
        if value == 0.0 {
            return if value.is_sign_negative() { "-0.0" } else { "0.0" }.to_owned();
        }
        let sign = if value < 0.0 { "-" } else { "" };
        let scientific = format!("{:e}", value.abs());
        let (mantissa, exponent) = scientific.split_once('e').unwrap_or((&scientific, "0"));
        let digits = mantissa.replacen('.', "", 1);
        let power: i32 = exponent.parse().unwrap_or(0);
        let point = power + 1;
        let count = i32::try_from(digits.len()).unwrap_or(i32::MAX);
        let zeros = |n: i32| "0".repeat(usize::try_from(n).unwrap_or(0));
        if count <= point && point <= 16 {
            format!("{sign}{digits}{}.0", zeros(point - count))
        } else if 0 < point && point <= 16 {
            let (whole, fraction) = digits.split_at(usize::try_from(point).unwrap_or(0));
            format!("{sign}{whole}.{fraction}")
        } else if -5 < point && point <= 0 {
            format!("{sign}0.{}{digits}", zeros(-point))
        } else if count == 1 {
            format!("{sign}{digits}e{power}")
        } else {
            format!("{sign}{}.{}e{power}", &digits[..1], &digits[1..])
        }
    }
}

impl PartialEq for Binary64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Binary64 {}

impl std::fmt::Display for Binary64 {
    /// The shortest plain decimal that reads back as the value: `-0`, `0.0000001`, never an
    /// exponent. Guards, filters, invariants and orders compare this text by value.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
