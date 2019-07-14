#[derive(Copy, Clone)]
pub struct KvStore {}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {}
    }

    pub fn get(self, _key: String) -> Option<String>{
        Some("helloworld".to_string())
    }

    pub fn set(self, _key: String, _value: String) -> String {
        "helloworld".to_string()
    }

    pub fn remove(self, _key: String){
    }
}
