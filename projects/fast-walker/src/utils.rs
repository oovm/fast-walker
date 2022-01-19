use std::{
    fmt::{Formatter, Write},
    path::Path,
};

/// Convert a path to unix style path.
///
/// # Examples
///
/// ```
/// # use fast_walker::utils::to_unix_path;
/// let path = std::path::Path::new("C:\\P4Root\\test.txt");
/// ```
pub fn to_unix_path(path: &Path) -> String {
    #[cfg(target_os = "windows")]
    {
        path.to_string_lossy().trim_start_matches("\\\\?\\").replace("\\", "/")
    }
    #[cfg(not(target_os = "windows"))]
    {
        path.to_string_lossy().to_string()
    }
}

pub(crate) fn write_unix_path(f: &mut Formatter<'_>, path: &Path) -> std::fmt::Result {
    for c in path.to_string_lossy().trim_start_matches("\\\\?\\").chars() {
        if c == '\\' { f.write_char('/')? } else { f.write_char(c)? }
    }
    Ok(())
}
