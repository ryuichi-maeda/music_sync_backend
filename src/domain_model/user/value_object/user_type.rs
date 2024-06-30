use async_graphql::Enum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum UserType {
    Guest,
    Registered,
}
