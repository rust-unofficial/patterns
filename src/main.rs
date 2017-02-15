//! Building scripts
use std::process::Command;


fn main() {
    println!("Building start...");
    testing();
    rendering();
    println!("Building complete.");
}

fn testing() {
    print!("Testing...");
    let mut test_proc = Command::new("mdbook").arg("test").spawn()
                                .expect("Failed to start the testing process");
    let ecode = test_proc.wait().expect("Failed to finish the testing process");
    assert!(ecode.success());
    println!("Done.");
}

fn rendering() {
    print!("Rendering...");
    let mut render_proc = Command::new("mdbook").arg("build").spawn()
                                .expect("Failed to start the rendering process");
    let ecode = render_proc.wait().expect("Failed to finish the rendering process");
    assert!(ecode.success());
    post_rendering();
    println!("Done.");
}

fn post_rendering() {
    let cdir = std::env::current_dir().expect("WTF current directory does not exist");
    process_dir(cdir, cdir);
    use std::fs::*;
    use std::path::Path;
    const BOOK_DIR_NANE = "book";
    fn process_dir(path: &Path, root: &Path) {
        if let Ok(entries) = read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            let file_path = entry.path();
                            if let Some(name) = file_path.file_name() {
                                if name != BOOK_DIR_NAME {
                                    process_dir(entry.path().as_path(), root);
                                }
                            }
                            
                        } else if file_type.is_file() {
                            let file_path = entry.path();
                            if let Some(ext) = file_path.extension() {
                                if ext == "html" {
                                    process_html(file_path.as_path(), root);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn process_html(path: &Path, root: &Path) {
        const BOOK_DIR_NANE = "book";
        let sub_path = path.strip_prefix(root).expect("invalid call");
        let mut new_path = root.to_owned();
        new_path.push(BOOK_DIR_NAME);
        new_path.push(sub_path);
        rename(path, new_path).expect("Failed to rename a file");
    }
}