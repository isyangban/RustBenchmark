use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sled;
use std::iter;
use std::path::Path;
use std::vec;
use tempfile;
use uuid::Uuid;

#[derive(Debug, Copy, Clone)]
pub struct BenchmarkConfig {
    pub name: &'static str,
    pub size: u32,
}

static CONFIGS: &'static [BenchmarkConfig] = &[
    BenchmarkConfig {
        name: "128B",
        size: 128,
    },
    BenchmarkConfig {
        name: "256B",
        size: 256,
    },
];

static NUM_TRIAL: i32 = 10;
static DEFAULT_KEY_COUNT: i32 = 1000;

#[derive(Debug)]
pub struct BenchmarkEnvironment {
    pub db: sled::Db,
    pub config: BenchmarkConfig,
    pub tempdir: tempfile::TempDir,
    pub sample_data: Vec<u8>,
    pub sample_key: String,
}

// Setup Required Environment
// DB Test 용으로 쓸 데이터는 미리 만들어 놓는 용도
pub fn setup_benchmark() -> Vec<BenchmarkEnvironment> {
    let mut environments: Vec<BenchmarkEnvironment> = vec![];
    for config in CONFIGS.iter() {
        let dir = tempfile::tempdir().unwrap();
        let (db, sample_key, sample_data) = create_db(&config, &dir.path());
        environments.push(BenchmarkEnvironment {
            db,
            sample_key,
            sample_data,
            config: *config,
            tempdir: dir,
        })
    }
    environments
}

// Create Tree (Make Random Tree)
fn create_db(config: &BenchmarkConfig, tempdir_path: &Path) -> (sled::Db, String, Vec<u8>) {
    let db = sled::open(tempdir_path.join(format!("sled_{}", config.name))).expect("open");
    let mut sample_data = String::from("data").into_bytes();
    let mut sample_key = String::from("key");
    for i in 1..DEFAULT_KEY_COUNT {
        let key = Uuid::new_v4().to_string();
        let data = create_random_string(config.size).into_bytes();
        if i == 1 {
            sample_key = key.clone();
            sample_data = data.clone();
        }
        db.insert(key, data);
    }
    (db, sample_key, sample_data)
}

fn create_random_string(len: u32) -> String {
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(len as usize)
        .collect();
    chars
}

// Benchmark Reading a specific key n times
pub fn benchmark_get(env: &BenchmarkEnvironment) {
    for _ in 1..NUM_TRIAL {
        let data = env.db.get(&env.sample_key);
        let sample_data = env.sample_data.clone();
        assert_eq!(data, Ok(Some(sled::IVec::from(sample_data))));
    }
}

// Benchmark scaning all the data (key, value)
pub fn benchmark_scan() {}

// Benchmark insterting all the data (key, value)
pub fn benchmark_insert() {}

// Benchmark Removing a specific key n times
pub fn benchmark_remove() {}

// Benchmark batch processing
