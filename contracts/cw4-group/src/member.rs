use cosmwasm_schema::cw_serde;

/// A group member has a weight associated with them.
/// This may all be equal, or may have meaning in the app that
/// makes use of the group (eg. voting power)
#[cw_serde]
pub struct Member {
    pub addr: String,
    pub weight: u64,
    pub identity: String,
}

#[cw_serde]
pub struct MemberListResponse {
    pub members: Vec<Member>,
}

#[cw_serde]
pub struct MemberResponse {
    pub weight: Option<u64>,
    pub identity: Option<String>,
}
