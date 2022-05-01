use std::{path::PathBuf, time::Instant};

use chrono::prelude::*;
use clap::{Parser, Subcommand};
use mdb_code_insights::{
    db::{FileChange, GitCommit, MongoInstance},
    git::GitProxy,
};

const DB_NAME: &str = "code_insights";
const COLL_NAME: &str = "commits";

#[derive(Debug, Subcommand)]
enum CommandType {
    Load {
        #[clap(long, parse(from_os_str), default_value = ".")]
        /// Path to repository to read.
        repo_dir: PathBuf,

        #[clap(long)]
        /// Cutoff date to look back [format="YYYY-MM-DD"].
        after_date: String,
    },
    FilesPerCommit,
    FileCoupling {
        #[clap(long)]
        /// Filename to query on.
        filename: String,
    },
    FileOwnership {
        #[clap(long)]
        /// Filename to query on.
        filename: String,
    },
    MostActiveFiles {
        #[clap(long)]
        /// Look at files touched since this date.
        since: Option<String>,

        #[clap(long)]
        /// file prefix to filter on.
        prefix: Option<String>,
    },
}

impl CommandType {
    pub async fn execute(&self, mongo: MongoInstance) {
        match self {
            CommandType::Load {
                repo_dir,
                after_date,
            } => {
                let now = Instant::now();
                let git = GitProxy::new(repo_dir);
                let output = git.log(&after_date);
                eprintln!("Git Query in: {}s", now.elapsed().as_secs());

                let now = Instant::now();
                let mut commit_list = vec![];
                let mut current_commit: Option<GitCommit> = None;
                let mut file_list = vec![];
                for line in output.lines() {
                    if !line.trim().is_empty() {
                        if line.starts_with("--") {
                            if let Some(mut commit) = current_commit {
                                commit.files = file_list;
                                commit_list.push(commit.clone());
                                file_list = vec![];
                            }
                            let parts: Vec<&str> = line.split("--").collect();
                            current_commit = Some(GitCommit {
                                commit: parts[1].to_string(),
                                date: iso_date_to_datetime(parts[2]),
                                author: parts[3].to_string(),
                                summary: parts[4].to_string(),
                                files: vec![],
                            });
                        } else {
                            if current_commit.is_some() {
                                let parts: Vec<&str> = line.split_ascii_whitespace().collect();
                                file_list.push(FileChange {
                                    added: parts[0].parse().unwrap_or_default(),
                                    deleted: parts[1].parse().unwrap_or_default(),
                                    filename: parts[2].to_string(),
                                });
                            }
                        }
                    }
                }
                eprintln!("Create data in: {}ms", now.elapsed().as_millis());
                println!("Loaded {} commits!", commit_list.len());

                let now = Instant::now();
                mongo.insert_commits(&commit_list).await.unwrap();
                eprintln!("Sent data to mongo in: {}ms", now.elapsed().as_millis());
            }
            CommandType::FilesPerCommit => {
                let results = mongo.file_per_commit().await.unwrap();
                for item in results {
                    println!("{}({}): {}", item._id, item.n_commits, item.avg_files);
                }
            }
            CommandType::FileCoupling { filename } => {
                let results = mongo.file_coupling(filename).await.unwrap();
                for item in results {
                    let total = item.total_commits[0].commit;
                    println!("{}: {} instances", filename, total);
                    println!("");
                    for x in item.seen_with {
                        let percent = x.count as f64 / total as f64 * 100.0;
                        println!(" - {}: {}: {:.02}%", x._id, x.count, percent);
                    }
                }
            }
            CommandType::FileOwnership { filename } => {
                let results = mongo.file_ownership(filename).await.unwrap();
                let total: u64 = results.iter().map(|r| r.count).sum();
                println!("Owners of {}: {} total changes", filename, total);
                for item in results {
                    let percent = item.count as f64 / total as f64 * 100.0;
                    println!("{}: {} ({:.02}%)", item._id, item.count, percent);
                }
            }
            CommandType::MostActiveFiles { since, prefix } => {
                let since_date = since.as_ref().map(|s| iso_date_to_datetime(s));
                let results = mongo
                    .file_activity(since_date.as_ref(), prefix.as_deref())
                    .await
                    .unwrap();
                println!("Most active files:");
                for item in results {
                    println!("{}: {}", item._id, item.count);
                }
            }
        }
    }
}

fn iso_date_to_datetime(iso_date: &str) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        NaiveDate::parse_from_str(iso_date, "%Y-%m-%d")
            .unwrap()
            .and_hms(0, 0, 0),
        Utc,
    )
}

#[derive(Debug, Parser)]
struct Args {
    /// URI to mongodb instance.
    #[clap(long, default_value = "mongodb://localhost:27017")]
    mongo_uri: String,

    /// Database to use.
    #[clap(long, default_value = DB_NAME)]
    database: String,

    /// Collection to use.
    #[clap(long, default_value = COLL_NAME)]
    collection: String,

    #[clap(subcommand)]
    /// Subcommand to execute.
    command: CommandType,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mongo = MongoInstance::new(&args.mongo_uri, &args.database, &args.collection)
        .await
        .unwrap();
    args.command.execute(mongo).await;
}
