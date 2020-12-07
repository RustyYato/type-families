pub enum Validity<T, E> {
    Valid(T),
    Invalid(E),
}
