pub struct Database {
    data: Vec<Vec<u8>>,
    total_length: usize,
}

impl From<&[String]> for Database {
    fn from(v: &[String]) -> Self {
        let mut data: Vec<Vec<u8>> = vec![];
        let mut total_length: usize = 0;

        for s in v {
            data.push(s.as_bytes().to_vec());
            total_length += data.last().unwrap().len();
        }

        Database { data, total_length }
    }
}
