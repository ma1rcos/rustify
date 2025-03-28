use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum Role {
    Master,
    Administrator,
    Photographer,
    Customer,
}

// impl Role {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             Role::Master => "Master",
//             Role::Administrator => "Administrator",
//             Role::Photographer => "Photographer",
//             Role::Customer => "Customer",
//         }
//     }
// }