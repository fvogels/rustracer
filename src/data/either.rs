#[derive(Debug, PartialEq, Eq)]
pub enum Either<T, U> {
    Left(T),
    Right(U),
}
