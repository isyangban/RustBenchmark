use std::iter;
use std::path::Path;
use std::vec;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use sled;
use tempfile;
use uuid::Uuid;


#[derive(Debug, Copy, Clone)]
pub struct BenchmarkConfig {
    pub name: &'static str,
    pub size: u32
}

static Configs: &'static [BenchmarkConfig] = &[
    BenchmarkConfig{name: "128B", size: 128},
    BenchmarkConfig{name: "256B", size: 256},
];

static NUM_TRIAL: i32 = 10;
static DEFAULT_KEY_COUNT: i32 = 1000;

#[derive(Debug)]
pub struct BenchmarkEnvironment {
    pub db: sled::Db,
    pub config: BenchmarkConfig,
    pub tempdir: tempfile::TempDir,
    pub sample_data: String,
    pub sample_key: String 
}

// Setup Required Environment
// DB Test 용으로 쓸 데이터는 미리 만들어 놓는 용도
pub fn setupBenchmark() -> Vec<BenchmarkEnvironment> {
    let mut environments: Vec<BenchmarkEnvironment> = vec![];
    for config in Configs.iter() {
        let dir = tempfile::tempdir().unwrap();
        let (db, sample_key, sample_data) = createDb(&config, &dir.path());
        environments.push(BenchmarkEnvironment{
            db, sample_key, sample_data, config: *config, tempdir: dir
        })
    } 
    environments
}

// Create Tree (Make Random Tree)
fn createDb(config: &BenchmarkConfig, tempdirPath: &Path) -> (sled::Db, String, String) {
    let db = sled::open(tempdirPath.join(format!("sled_{}", config.name))).unwrap();
    let mut sampleData = "data".to_string();
    let mut sampleKey = "key".to_string();
    for i in 1..DEFAULT_KEY_COUNT {
        let key = Uuid::new_v4().to_string();
        let data = createRandomString(config.size);
        db.insert("sdfsdf", &data);
        if i == 1 {
            sampleKey = key;
            sampleData = data;
        }
    }
    (db, sampleKey, sampleData)
}

fn createRandomString(len: u32) -> String {
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(len as usize)
        .collect();
   chars 
}

// Benchmark Reading a specific key n times
pub fn BenchmarkGet(env: &mut BenchmarkEnvironment) {
    for _ in 1..NUM_TRIAL {
        let data = env.db.get(&env.sample_key);
        assert_eq!(data, Ok(Some(&env.sample_data)));
    }

}

// Benchmark scaning all the data (key, value) 
pub fn BenchmarkScan() {

}

// Benchmark insterting all the data (key, value) 
pub fn BenchmarkInsert() {

}

// Benchmark Removing a specific key n times
pub fn BenchmarkRemove() {

}

// Benchmark batch processing