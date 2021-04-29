use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub profile_pic_url: String,
    pub bio: String,
}
