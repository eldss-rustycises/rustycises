use super::schema::urls;

/// Url represents a mapping from a shortened url to the actual url.
#[derive(Insertable)]
#[table_name = "urls"]
pub struct NewUrl<'a> {
    pub short: &'a str,
    pub long: &'a str,
}

/// Full representation of a row in the urls table.
#[derive(Queryable, Identifiable)]
pub struct Url {
    pub id: i32,
    pub short: String,
    pub long: String,
}
