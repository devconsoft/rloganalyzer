
#[derive(Clone, PartialEq, Debug, Eq)]
pub struct Data {
    index: usize,
    content: String,
}

impl Data {

    pub fn new(index: usize, content: String) -> Data {
        Data {
            index,
            content,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Data};

    #[test]
    fn data() {
        let d = Data::new(1, "content".to_string());
        assert_eq!(d.index, 1);
        assert_eq!(d.content, "content");
    }
}
