pub trait GetIdTrait {
    type IdType: PartialEq;

    fn get_id(&self) -> Self::IdType;
}
