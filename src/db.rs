use anyhow::Result;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use mongodb::{
    bson::{self, doc, Document},
    error::ErrorKind,
    options::{ClientOptions, IndexOptions, InsertManyOptions},
    Client, Collection, IndexModel,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

const DUPLICATE_KEY_ERR: i32 = 11000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub added: u64,
    pub deleted: u64,
    pub filename: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCommit {
    pub commit: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub date: DateTime<Utc>,
    pub author: String,
    pub summary: String,
    pub files: Vec<FileChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesPerCommit {
    pub _id: String,
    pub avg_files: f64,
    pub n_commits: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeenWith {
    pub _id: String,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitCount {
    pub commit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCoupling {
    pub total_commits: Vec<CommitCount>,
    pub seen_with: Vec<SeenWith>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOwnership {
    pub _id: String,
    pub count: u64,
}

pub struct MongoInstance {
    collection: Collection<GitCommit>,
}

impl MongoInstance {
    pub async fn new(mongo_uri: &str, database: &str, collection: &str) -> Result<Self> {
        let client_options = ClientOptions::parse(mongo_uri).await?;
        let client = Client::with_options(client_options)?;
        let database = client.database(database);
        Ok(Self {
            collection: database.collection(collection),
        })
    }

    pub async fn create_indexes(&self) -> Result<()> {
        let index_options = IndexOptions::builder().unique(true).build();
        let index_model = IndexModel::builder()
            .keys(doc! { "commit": 1_i32 })
            .options(index_options)
            .build();
        self.collection.create_index(index_model, None).await?;

        let index_model = IndexModel::builder().keys(doc! {"date": -1_i32}).build();
        self.collection.create_index(index_model, None).await?;

        let index_model = IndexModel::builder()
            .keys(doc! {"date": -1_i32, "files.filename": 1_i32})
            .build();
        self.collection.create_index(index_model, None).await?;

        let index_model = IndexModel::builder()
            .keys(doc! {"files.filename": 1_i32})
            .build();
        self.collection.create_index(index_model, None).await?;

        let index_model = IndexModel::builder().keys(doc! {"author": 1_i32}).build();
        self.collection.create_index(index_model, None).await?;
        Ok(())
    }

    pub async fn insert_commits(&self, commit_list: &[GitCommit]) -> Result<()> {
        let options = InsertManyOptions::builder().ordered(false).build();
        let result = self
            .collection
            .insert_many(commit_list, Some(options))
            .await;
        if let Err(error) = &result {
            if let ErrorKind::BulkWrite(bulk_write_failures) = &error.kind.as_ref() {
                if let Some(failure_list) = &bulk_write_failures.write_errors {
                    if !failure_list.iter().any(|f| f.code != DUPLICATE_KEY_ERR) {
                        return Ok(());
                    }
                }
            }
            dbg!(&result);
            result?;
        }
        Ok(())
    }

    pub async fn file_per_commit(&self) -> Result<Vec<FilesPerCommit>> {
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

        self.aggregate(pipeline).await
    }

    pub async fn file_coupling(
        &self,
        filename: &str,
        since_date: Option<&DateTime<Utc>>,
    ) -> Result<Vec<FileCoupling>> {
        let filter = if let Some(since_date) = since_date {
            doc! { "$match": {
                "date": {"$gt": since_date},
                "files.filename": filename,
            }}
        } else {
            doc! {
                "$match": {
                    "files.filename": filename,
                }
            }
        };

        let pipeline = vec![
            filter,
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
                            "$sortByCount": "$files.filename"
                        },
                    ],
                }
            },
        ];

        self.aggregate(pipeline).await
    }

    pub async fn file_ownership(
        &self,
        filename: &str,
        since_date: Option<&DateTime<Utc>>,
    ) -> Result<Vec<FileOwnership>> {
        let pipeline = [
            doc! { "$match": {
                "date": {"$gt": since_date},
                "files.filename": filename,
            }},
            doc! {"$sortByCount": "$author"},
        ];

        self.aggregate(pipeline).await
    }

    pub async fn file_activity(
        &self,
        since_date: Option<&DateTime<Utc>>,
        file_prefix: Option<&str>,
    ) -> Result<Vec<FileOwnership>> {
        let mut pipeline = create_filter(since_date, file_prefix);
        pipeline.push(doc! {
            "$sortByCount": "$files.filename"
        });

        self.aggregate(pipeline).await
    }

    async fn aggregate<T>(&self, pipeline: impl IntoIterator<Item = Document>) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let mut results = self.collection.aggregate(pipeline, None).await?;
        let mut items = vec![];
        while let Some(result) = results.next().await {
            items.push(bson::from_document(result?)?);
        }
        Ok(items)
    }
}

fn create_filter(since_date: Option<&DateTime<Utc>>, file_prefix: Option<&str>) -> Vec<Document> {
    let mut filter = vec![];
    if let Some(since_date) = since_date {
        filter.push(doc! { "$match": {"date": {"$gt": since_date}}});
    }
    filter.push(doc! {"$unwind": {"path": "$files"}});

    if let Some(file_prefix) = file_prefix {
        let re_match = format!("^{}", file_prefix);
        filter.push(doc! {"$match": {"files.filename": {"$regex": re_match}}});
    }

    filter
}
