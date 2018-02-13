#[derive(Debug)]
pub struct Tags {
    pub tags: Vec<Tag>
}

#[derive(Debug)]
pub struct Tag {
    pub value: String
}

impl Tags {
    pub fn empty() -> Tags {
        Tags {
            tags: Vec::with_capacity(0)
        }
    }
}
