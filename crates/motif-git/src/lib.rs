use chrono::{DateTime, NaiveDateTime, Utc};
use git2::{Repository, Time};
use std::error::Error;
use std::path::Path;

pub fn run(repo_path: &Path) -> Result<(), Box<dyn Error>> {
    // Open the repository
    let repo = Repository::open(repo_path)?;

    // Specify the date for which you want to find commits
    let target_date = Time::new(1661990400, 0); // Replace with the specific Unix timestamp

    // Iterate through all commits and filter by date
    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        let author = commit.author();

        let var_name = author.when().seconds() == target_date.seconds();
        if true {
            println!("Commit: {}", commit.id());

            // To collect diff lines, you need to compare the current commit with its parent
            if commit.parent_count() > 0 {
                let parent = commit.parent(0)?;
                let diff =
                    repo.diff_tree_to_tree(Some(&parent.tree()?), Some(&commit.tree()?), None)?;

                diff.print(git2::DiffFormat::Patch, |_, _, line| {
                    print!("{}", std::str::from_utf8(line.content()).unwrap());
                    true
                })?;
            }
            break;
        }
    }

    Ok(())
}

fn datetime(git_time: Time) {
    let naive_datetime = NaiveDateTime::from_timestamp(git_time.seconds(), 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
}
