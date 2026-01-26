use crate::impl_table_row;

#[derive(Debug)]
pub struct SshKeyRow {
    pub id: String,
    pub name: String,
    pub login: String,
    pub created: String,
}

impl_table_row!(SshKeyRow, id, name, login, created);
