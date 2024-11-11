use strum::EnumIs;

#[derive(
    Debug,
    PartialEq,
    Clone,
    Copy,
    EnumIs,
    serde::Serialize,
    serde::Deserialize,
    sqlx::Type,
    strum::Display,
)]
#[sqlx(type_name = "Text")]
pub enum Scopes {
    /// An Admin has unrestricted access to the system.
    Admin,
    /// A user who can read participants.
    ///
    /// - View Demographics
    /// - View Contact Information
    /// - View Case Notes
    ReadParticipants,
    /// A user who can update participants.
    ///
    /// - Update Demographics
    /// - Update Contact Information
    /// - Add or Update Case Notes
    UpdateParticipants,
    /// A user who can add participants.
    AddParticipants,
    /// A user who can add, update, remove users
    ManageUsers,
}
