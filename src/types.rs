
use util;
use database::schematic::clients;


#[derive(Eq, PartialEq, FromForm, Serialize, Queryable)]
pub struct InviteForm {
    invite: String
}

impl InviteForm {
    pub fn get_invite(&self) -> String {
        self.invite
    }

    pub fn from(invite: String) -> InviteForm {
        InviteForm {
            invite
        }
    }

}

#[derive(Serialize)]
pub struct SerializableResult<T> {
    error: Option<String>,
    data: Option<T>,
}

#[derive(Serialize)]
pub struct RequestJoinData {
    identity: User, // DO NOT LOSE / LEAK.
}

#[derive(Serialize)]
pub struct FileDetails {
    owner: User,
    id: String,
    ipfs: String,
    link: String
}

#[derive(Serialize, Insertable)]
#[table_name="clients"]
pub struct User {
    id: i32,
    identity: String
}


impl User {
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_identity(&self) -> String {
        self.identity
    }
    pub fn create(last_id: i32) -> User {
        User {
            id: last_id + 1,
            identity: util::random_string(32),
        }
    }
}