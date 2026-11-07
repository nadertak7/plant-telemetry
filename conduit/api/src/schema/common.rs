use serde::Deserialize;

#[derive(Deserialize)]
pub struct ObjectWithId {
    pub id: i32,
}
