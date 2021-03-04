use atty::Stream;
use std::fmt;

#[cfg(not(target_os = "macos"))]
lazy_static::lazy_static! {
    static ref IS_LANG_UTF8: bool = {
        match std::env::var("LANG") {
            Ok(lang) => lang.to_uppercase().ends_with("UTF-8"),
            _ => false,
        }
    };
}

/// An emoji with safety fallback.
///
/// The struct wraps an emoji and only renders it on platforms that actually
/// support it. On non-supported platforms the fallback value is being rendered.
///
/// Support is determined by two factors:
///
/// 1) The processes stdout has to be a tty.
/// 2) Platform dependent:
///     - macOS has emoji support by default
///     - Unix systems have support if the active language supports them.
///     - Windows machines running the new Terminal app support emojis.
pub struct Emoji<'a, 'b>(pub &'a str, pub &'b str);

impl<'a, 'b> Emoji<'a, 'b> {
    /// Create a new emoji.
    ///
    /// # Arguments
    ///
    /// - `emoji`: The unicode emoji to display.
    /// - `fallback`: The fallback value to use on non-supported platforms.
    pub const fn new(emoji: &'a str, fallback: &'b str) -> Self {
        Self(emoji, fallback)
    }
}

impl fmt::Display for Emoji<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if should_display_emoji() {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}", self.1)
        }
    }
}

impl<'a, 'b> From<(&'a str, &'b str)> for Emoji<'a, 'b> {
    fn from(v: (&'a str, &'b str)) -> Self {
        Emoji(v.0, v.1)
    }
}

// Emojis should only get displayed if the current terminal is a tty and the
// platform does support emojis.
fn should_display_emoji() -> bool {
    atty::is(Stream::Stdout) && is_emoji_supported()
}

// The new Windows Terminal does support emojis. Currently, the terminal will
// set the environment variable `WT_SESSION`. This can be used to check if the
// user uses that specific app.
#[cfg(windows)]
fn is_emoji_supported() -> bool {
    std::env::var("WT_SESSION").is_ok()
}

// macOS by default has emoji support.
#[cfg(target_os = "macos")]
fn is_emoji_supported() -> bool {
    true
}

// On unix systems the enabled language decides whether emojis are supported or
// not.
#[cfg(all(unix, not(target_os = "macos")))]
fn is_emoji_supported() -> bool {
    *IS_LANG_UTF8
}
