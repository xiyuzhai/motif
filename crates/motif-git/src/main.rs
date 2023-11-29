use motif_git::run;
use std::path::Path;

fn main() {
    let repo_path = Path::new(".");
    run(repo_path).unwrap()
}
