use async_graphql::Enum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum UserType {
    Guest,
    Registered,
}

impl From<String> for UserType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "guest" => UserType::Guest,
            "registered" => UserType::Registered,
            _ => panic!("Invalid user type: {}", s),
        }
    }
}
