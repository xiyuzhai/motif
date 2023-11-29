use colored::{Color, Colorize, CustomColor};
use motif_git::{commits_stats_for_last_k_days_excluding_today, CommitStats};
use motif_hash::hash_to_u64;
use motif_one_drop::{OneDropGenerator, Rune};
#[allow(deprecated)]
use std::{env::home_dir, path::PathBuf};
use std::{thread::sleep, time::Duration};

const REPO_NAMES: &[&str] = &["positional-expressive-power-of-transformers"];

#[allow(deprecated)]
fn repo_paths() -> Vec<PathBuf> {
    let repos = home_dir().expect("should be okay").join("repos");
    REPO_NAMES.iter().map(|name| repos.join(name)).collect()
}

const K: i64 = 1;

const SEASON_SEED: u64 = 1489178819;

fn main() {
    for repo_path in repo_paths() {
        println!("Drop runes from repo `{:?}`", repo_path);
        for commit_stats in
            commits_stats_for_last_k_days_excluding_today(&repo_path, K).expect("should be okay")
        {
            let mut generator =
                OneDropGenerator::new(SEASON_SEED + hash_to_u64(&commit_stats.id), 0.02, 0.83);
            let mut rune: Option<Rune> = None;
            for _ in 0..commit_stats.nline_deltas {
                rune = rune.max(generator.generate())
            }
            match rune {
                Some(rune) => println!(
                    "    {} dropped from commit `{:?}`",
                    format!("{:?}", rune).color(Color::TrueColor {
                        r: 153,
                        g: 101,
                        b: 45
                    }),
                    commit_stats.id
                ),
                None => println!("\tno drop"),
            }
            sleep(Duration::from_secs(1));
        }
    }
}
