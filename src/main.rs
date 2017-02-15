//! Building scripts
use std::process::Command;
use std::path::{Path, PathBuf};

fn main() {
    println!("Building start...");
    //testing();
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
    let (olds, news) = before_rendering();
    let mut render_proc = Command::new("mdbook").arg("build").spawn()
                                .expect("Failed to start the rendering process");
    let ecode = render_proc.wait().expect("Failed to finish the rendering process");
    assert!(ecode.success());
    post_rendering(olds, news);
    println!("Done.");
}

fn before_rendering() -> (Vec<PathBuf>, Vec<PathBuf>) {
    let root = std::env::current_dir().expect("WTF current directory does not exist");
    let file_paths = {
            fn file_paths(spath: &Path) -> Vec<String> {
                use std::io::{Read, Write};
                let scontent = {
                let sfile = std::fs::File::open(spath)
                    .expect("Failed to open SUMMARY.md");
                let mut buf = String::with_capacity(sfile.metadata().unwrap().len() as usize);
                (&sfile).read_to_string(&mut buf).expect("Failed to read from SUMMARY.md");
                buf
            };

            let mut paths = Vec::new();
            for line in scontent.lines() {
                if let Some(index) = line.find("(./") {
                    let (_, path) = line.split_at(index+3);
                    paths.push(path[..path.len()-1].to_owned());
                }
            }
            paths
        }
        let mut summary_path = root.to_path_buf();
        summary_path.push("src");
        summary_path.push("SUMMARY.md");
        file_paths(summary_path.as_path())
    };
    let mut old_paths = Vec::new();
    let mut new_paths = Vec::new();
    for path in &file_paths {
        let mut old = root.to_path_buf();
        old.push(&path);
        let mut new = root.to_path_buf();
        new.push("src");
        new.push(&path);
        std::fs::create_dir_all(new.parent().unwrap())
            .expect("Failed to move a file");
        std::fs::rename(old.as_path(), new.as_path())
            .expect("Failed to move a file");
        old_paths.push(old);
        new_paths.push(new);
    }
    (old_paths, new_paths)
}
fn post_rendering(olds: Vec<PathBuf>, news: Vec<PathBuf>) {
    assert_eq!(olds.len(), news.len());
    for i in 0..olds.len() {
        std::fs::rename(news[i].as_path(), olds[i].as_path())
            .expect("Failed to move back!");
    }
}