pub struct KvStore {}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {}
    }

    pub fn get(self, key: String) -> String {
        "helloworld".to_string()
    }

    pub fn set(self, key: String, value: String) -> String {
        "helloworld".to_string()
    }
}