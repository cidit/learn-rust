pub trait DataAccessObject<T> {
    type Identity;
    type Error;

    fn get_one(&self, id: Self::Identity) -> Result<Option<T>, Self::Error>;

    fn get_all(&self) -> Result<Vec<T>, Self::Error>;

    fn save(&self, item: T) -> Result<Self::Identity, Self::Error>;
}
