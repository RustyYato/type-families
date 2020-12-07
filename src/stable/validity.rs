use super::*;
use crate::validity::Validity;

use ghost::phantom;

#[phantom]
#[derive(Debug, Clone, Copy)]
pub struct ValidityFamily<E>;

impl<T, E> Family<T> for ValidityFamily<E> {
    type This = Validity<T, E>;
}

impl<T, E> Family<E> for ValidityFamilyInvalid<T> {
    type This = Validity<T, E>;
}

impl<A, B, E> Functor<A, B> for ValidityFamily<E> {
    fn map<F: Fn(A) -> B>(self, this: This<Self, A>, f: F) -> This<Self, B> {
        match this {
            Validity::Valid(valid) => Validity::Valid(f(valid)),
            Validity::Invalid(invalid) => Validity::Invalid(invalid),
        }
    }
}

impl<A, E> Pure<A> for ValidityFamily<E> {
    fn pure(self, value: A) -> This<Self, A> { Validity::Valid(value) }
}

impl<A, B, E: SemiGroup> Applicative<A, B> for ValidityFamily<E> {
    fn lift_a2<C, F>(self, a: This<Self, A>, b: This<Self, B>, f: F) -> This<Self, C>
    where
        F: Fn(A, B) -> C,
    {
        match (a, b) {
            (Validity::Valid(a), Validity::Valid(b)) => Validity::Valid(f(a, b)),
            (Validity::Invalid(a), Validity::Invalid(b)) => Validity::Invalid(a.append(b)),
            (Validity::Invalid(x), _) | (_, Validity::Invalid(x)) => Validity::Invalid(x),
        }
    }
}

impl<A, B, E: SemiGroup> Monad<A, B> for ValidityFamily<E> {
    fn bind<F>(self, a: This<Self, A>, f: F) -> This<Self, B>
    where
        F: Fn(A) -> This<Self, B>,
    {
        match a {
            Validity::Valid(value) => f(value),
            Validity::Invalid(error) => Validity::Invalid(error),
        }
    }
}

impl<A, B, F, E> Traverse<A, B, F> for ValidityFamily<E>
where
    F: Applicative<B, This<Self, B>>,
{
    fn traverse<G>(self, app: F, this: This<Self, A>, g: G) -> This<F, This<Self, B>>
    where
        G: Fn(A) -> This<F, B> + Copy,
    {
        match this {
            Validity::Valid(value) => app.map(g(value), Validity::Valid),
            Validity::Invalid(error) => app.pure(Validity::Invalid(error)),
        }
    }
}
