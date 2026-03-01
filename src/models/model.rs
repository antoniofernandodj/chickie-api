use uuid::Uuid;

pub trait Model {
    fn get_uuid(&self) -> Uuid;
    fn set_uuid(&mut self, uuid: Uuid);
}