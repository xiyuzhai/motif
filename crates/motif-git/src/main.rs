use std::path::Path;

use motif_git::commits_stats_for_last_k_days_excluding_today;

fn main() {
    let repo_path = Path::new(".");
    println!(
        "last 5 days commits stats = {:?}",
        commits_stats_for_last_k_days_excluding_today(repo_path, 5).unwrap()
    )
}
