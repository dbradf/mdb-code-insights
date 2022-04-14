use std::{path::PathBuf, time::Instant};

use clap::{Parser, Subcommand};
use cmd_lib::run_fun;
use futures::StreamExt;
use mongodb::{options::ClientOptions, Client, bson::{doc, self}};
use serde::{Serialize, Deserialize};

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
    FilePerCommit,
    FileCoupling {
        #[clap(long)]
        /// Filename to query on.
        filename: String,
    }
}

impl CommandType {
    pub async fn execute(&self, mongo_uri: &str) {
        match self {
            CommandType::Load { repo_dir, after_date } => {
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
                                date: parts[2].to_string(),
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
                let client_options = ClientOptions::parse(mongo_uri).await.unwrap();
                let mdb_client = Client::with_options(client_options).unwrap();
                let db = mdb_client.database(DB_NAME);
                let collection = db.collection::<GitCommit>(COLL_NAME);

                collection.insert_many(commit_list, None).await.unwrap();
                eprintln!("Sent data to mongo in: {}ms", now.elapsed().as_millis());
            }
            CommandType::FilePerCommit => {
                let client_options = ClientOptions::parse(mongo_uri).await.unwrap();
                let mdb_client = Client::with_options(client_options).unwrap();
                let db = mdb_client.database(DB_NAME);
                let collection = db.collection::<GitCommit>(COLL_NAME);

                let pipeline = vec![
                    doc! {
                        "$addFields": {"file_count": {"$size": "$files"}}
                    },
                    doc! {
                        "$group": {
                            "_id": "$author", 
                            "avg_files": {"$avg": "$file_count"}, 
                            "n_commits": {"$count": {}}
                        }
                    },
                    doc! {
                        "$sort": {"avg_files": -1_i32}
                    },
                ];

                let mut results = collection.aggregate(pipeline, None).await.unwrap();
                while let Some(result) = results.next().await {

                    let item: FilesPerCommit = bson::from_document(result.unwrap()).unwrap();
                    println!("{}({}): {}", item._id, item.n_commits, item.avg_files);
                }

            }
            CommandType::FileCoupling { filename } => {
                let client_options = ClientOptions::parse(mongo_uri).await.unwrap();
                let mdb_client = Client::with_options(client_options).unwrap();
                let db = mdb_client.database(DB_NAME);
                let collection = db.collection::<GitCommit>(COLL_NAME);

                let pipeline = vec![
                    doc! {
                        "$match": {"files.filename": filename}
                    },
                    doc! {
                        "$facet": {
                            "total_commits": [{"$count": "commit"}],
                            "seen_with": [
                                {
                                    "$unwind": {"path": "$files"}
                                },
                                {
                                    "$match": {"files.filename": {"$ne": filename}}
                                },
                                {
                                    "$group": {
                                        "_id": "$files.filename", 
                                        "count": {"$sum": 1}
                                    }
                                },
                                {
                                    "$sort": {
                                        "count": -1
                                    }
                                }
                            ],
                        }
                    }
                ];

                let mut results = collection.aggregate(pipeline, None).await.unwrap();
                while let Some(result) = results.next().await {
                    let r = result.unwrap();
                    // dbg!(&r);
                    let item: FileCoupling = bson::from_document(r).unwrap();
                    let total = item.total_commits[0].commit;
                    println!("{}: {} instances", filename, total);
                    println!("");
                    for x in item.seen_with {
                        let percent = x.count as f64 / total as f64 * 100.0;
                        println!(" - {}: {}: {:.02}%", x._id, x.count, percent);

                    }
                }

            }
        }
    }
}

#[derive(Debug, Parser)]
struct Args {
    /// URI to mongodb instance.
    #[clap(long, default_value = "mongodb://localhost:27017")]
    mongo_uri: String,

    #[clap(subcommand)]
    /// Subcommand to execute.
    command: CommandType,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    args.command.execute(&args.mongo_uri).await;
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileChange {
    added: u64,
    deleted: u64,
    filename: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GitCommit {
    commit: String,
    date: String,
    author: String,
    summary: String,
    files: Vec<FileChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SeenWith {
    _id: String,
    count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CommitCount {
    commit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileCoupling {
    total_commits: Vec<CommitCount>,
    seen_with: Vec<SeenWith>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FilesPerCommit {
    _id: String,
    avg_files: f64,
    n_commits: u64,
}

struct GitProxy {
    working_dir: PathBuf,
}

impl GitProxy {
    pub fn new(working_dir: &PathBuf) -> Self {
        Self {
            working_dir: working_dir.to_path_buf(),
        }
    }

    pub fn log(&self, after_date: &str) -> String {
        let dir = &self.working_dir;
        run_fun!(
            cd $dir;
            git log --numstat --date=short --pretty=format:"--%h--%cd--%aN--%s" --no-renames --after=$after_date
        ).unwrap()
    }
}
