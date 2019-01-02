use uuid::Uuid;

macro_rules! id {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
        pub struct $name(pub String);

        impl $name {
            pub fn new(value: String) -> $name {
                $name(value)
            }

            pub fn generate() -> $name {
                $name(format!("{}", Uuid::new_v4()))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

id!(UserId);
id!(CategoryId);
id!(EntryId);
id!(SmsId);
