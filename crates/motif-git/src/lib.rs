use chrono::{DateTime, Days, Duration, NaiveDate, NaiveDateTime, Utc};
use git2::{Oid, Repository, Time};
use std::error::Error;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct CommitStats {
    pub id: Oid,
    pub nfile_deltas: usize,
    pub nline_deltas: usize,
}

pub fn commits_stats_for_last_k_days_excluding_today(
    repo_path: &Path,
    k: i64,
) -> Result<Vec<CommitStats>, Box<dyn Error>> {
    // Open the repository
    let repo = Repository::open(repo_path)?;

    // Specify the date for which you want to find commits
    let target_date = Time::new(1661990400, 0); // Replace with the specific Unix timestamp

    // Iterate through all commits and filter by date
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let mut commits_stats = vec![];

    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        let author = commit.author();
        let yesterday_naive_date = yesterday_naive_date();

        if (yesterday_naive_date - git_time_to_naive_date(author.when())) <= Duration::days(k) {
            // To collect diff lines, you need to compare the current commit with its parent
            // if commit.parent_count() > 0 {
            let parent_tree = if commit.parent_count() > 0 {
                Some(commit.parent(0)?.tree()?)
            } else {
                None
            };
            let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&commit.tree()?), None)?;
            let mut nfile_deltas = 0;
            let mut nline_deltas = 0;
            diff.foreach(
                &mut |_, _| {
                    nfile_deltas += 1;
                    true
                },
                None,
                None,
                Some(&mut |_, _, _| {
                    nline_deltas += 1;
                    true
                }),
            )?;
            commits_stats.push(CommitStats {
                id: commit.id(),
                nfile_deltas,
                nline_deltas,
            })
        }
    }

    Ok(commits_stats)
}

fn yesterday_naive_date() -> NaiveDate {
    Utc::now()
        .checked_sub_days(Days::new(0))
        .expect("yesterday exists!")
        .date_naive()
}

fn git_time_to_naive_date(git_time: Time) -> NaiveDate {
    let naive_datetime = NaiveDateTime::from_timestamp_opt(git_time.seconds(), 0).unwrap();
    DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc).date_naive()
}
