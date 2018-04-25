use types;
use database;
use rocket::request::Form;
use rocket_contrib::Json;

use diesel::RunQueryDsl;
use diesel::QueryDsl;

// ...
fn get_invites(conn: database::DBConnection) -> Vec<types::InviteForm> {
    use database::schematic::invites::dsl::*;
    invites.load::<types::InviteForm>(&*conn).unwrap() // unsafe...
}

fn add_user(conn: database::DBConnection, user: &types::User) {
    use diesel::insert_into;
    use database::schematic::clients::dsl::*;
    insert_into(clients)
        .values(user)
        .execute(&*conn);
}

fn get_last_user_id(conn: database::DBConnection) -> i32 {
    use database::schematic::clients::dsl::*;
    use diesel::dsl::max;
    clients.select(max(id))
        .first(&*conn).unwrap()
}

fn does_invite_exist(conn: database::DBConnection, invite: String) -> bool {
    get_invites(conn).contains(&types::InviteForm::from(invite))
}

pub fn try_join(conn: database::DBConnection, invite_form: types::InviteForm) -> types::SerializableResult<types::RequestJoinData> {
    let invite_str = invite_form.get_invite();
    /*
    if invite in invites() ...
        create db entry
       return thing.
    }
    */
    if does_invite_exist(conn, invite_str) {
        let last_id = get_last_user_id(conn);
        let identity = types::User::create(last_id);
        add_user(conn, &identity);
        return types::SerializableResult {
            error: None,
            data: Some(types::RequestJoinData {
                identity
            })
        }
    } else {
        return types::SerializableResult {
            error: Some(String::from("Invite does not exist.")),
            data: None
        }
    }
}

#[post("/api/join", data="<invite>")]
pub fn join(connection: database::DBConnection, invite: Form<types::InviteForm>) -> Json<types::SerializableResult<types::RequestJoinData>> {
    Json(try_join(connection, invite.into_inner()))
}
