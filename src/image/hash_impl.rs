use crate::image::Hash;

extern crate md5;

impl Hash for Vec<Vec<u8>> {
    fn hash(&self) -> String {
        let mut m = md5::Context::new();
        for row in self {
            m.consume(&row.as_slice());
        }
        let hash: Vec<String> = m.compute().iter().map(|x| format!("{:02x}", x)).collect();
        hash.join("")
    }
//    fn hash(&self) -> String {
//        let mut hasher = Sha1::new();
//        for row in self {
//            hasher.chain(row.as_slice());
//        }
//        let hash: Vec<String> = hasher.result().iter().map(|x| format!("{:02x}", x)).collect();
//        hash.join("")
//    }
}