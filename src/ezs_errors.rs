use thiserror::Error;

#[derive(Debug, Error)]
pub enum EzsError {
    #[error("Attempted to add a component to an entity without calling create_entity first")]
    CreateEntityNeverCalled,
    #[error("Attempted to insert data for a component that wasn't registered")]
    ComponentNotRegistered,
}
