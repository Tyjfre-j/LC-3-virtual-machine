use std::path::{Path, PathBuf};

/// Resolve a program image argument into a path.
///
/// Accepts:
/// - an existing path (e.g. `.\programs\2048.obj`)
/// - a filename in `.\programs` (e.g. `2048.obj`)
/// - a bare name (e.g. `2048` -> `.\programs\2048.obj`)
pub fn resolve_image_path(arg: &str) -> PathBuf {
    let p = Path::new(arg);
    if p.exists() {
        return p.to_path_buf();
    }

    let programs = Path::new("programs");
    let direct = programs.join(arg);
    if direct.exists() {
        return direct;
    }

    let with_obj = programs.join(format!("{arg}.obj"));
    if with_obj.exists() {
        return with_obj;
    }

    // Fallback: keep the original arg (we'll error nicely on open).
    p.to_path_buf()
}
