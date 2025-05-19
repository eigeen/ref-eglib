use std::path::{Component, Path, PathBuf};

pub fn normalize_path(input: impl AsRef<Path>) -> PathBuf {
    let input = input.as_ref();
    let current_dir = std::env::current_dir().expect("Failed to get current dir");

    let absolute_path = if input.is_absolute() {
        input.to_path_buf()
    } else {
        current_dir.join(input)
    };

    // 处理路径组件
    let mut stack = Vec::new();
    for component in absolute_path.components() {
        match component {
            Component::CurDir => {} // drop current dir
            Component::ParentDir => {
                // rollback
                if let Some(Component::Normal(_)) = stack.last() {
                    stack.pop();
                }
            }
            component => stack.push(component),
        }
    }

    // 重构路径
    let mut normalized = PathBuf::new();
    for component in stack {
        normalized.push(component.as_os_str());
    }

    // remove \\? prefix
    let path_str = normalized.to_str().unwrap();
    if path_str.starts_with("\\\\?\\") {
        let path = PathBuf::from(path_str.trim_start_matches("\\\\?\\"));
        return path;
    }

    normalized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path() {
        println!("{}", normalize_path(Path::new("foo/bar/..")).display());
        println!(
            "{}",
            normalize_path(Path::new("\\\\?\\C:\\foo\\bar\\..")).display()
        );
    }
}
