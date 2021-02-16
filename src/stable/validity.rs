use super::*;
use crate::{trait_aliases::CallOnce, validity::Validity};

use ghost::phantom;

#[phantom]
pub struct ValidityFamily<#[invariant] E>;

impl<E> Copy for ValidityFamily<E> {}
impl<E> Clone for ValidityFamily<E> {
    fn clone(&self) -> Self { *self }
}

impl<T, E> Family<T> for ValidityFamily<E> {
    type This = Validity<T, E>;
}

impl<A, B, E> Functor<A, B> for ValidityFamily<E> {
    type FuncBounds = CallOnce<A>;

    fn map<F, FTag>(self, this: This<Self, A>, f: F) -> This<Self, B>
    where
        Self::FuncBounds: Callable<F, FTag, Arg = A, Output = B>,
    {
        match this {
            Validity::Valid(valid) => Validity::Valid(CallOnce::build(f)(valid)),
            Validity::Invalid(invalid) => Validity::Invalid(invalid),
        }
    }
}

impl<A, E> Pure<A> for ValidityFamily<E> {
    fn pure(self, value: A) -> This<Self, A> { Validity::Valid(value) }
}

impl<A, B, E: SemiGroup> Applicative<A, B> for ValidityFamily<E> {
    type AppBounds = CallOnce<(A, B)>;

    fn lift_a2<C, F, FTag>(self, a: This<Self, A>, b: This<Self, B>, f: F) -> This<Self, C>
    where
        Self::AppBounds: Callable<F, FTag, Arg = (A, B), Output = C>,
    {
        match (a, b) {
            (Validity::Valid(a), Validity::Valid(b)) => Validity::Valid(CallOnce::build(f)((a, b))),
            (Validity::Invalid(a), Validity::Invalid(b)) => Validity::Invalid(a.append(b)),
            (Validity::Invalid(x), _) | (_, Validity::Invalid(x)) => Validity::Invalid(x),
        }
    }
}

impl<A, B, E: SemiGroup> Monad<A, B> for ValidityFamily<E> {
    fn bind<F, FTag>(self, a: This<Self, A>, f: F) -> This<Self, B>
    where
        Self::FuncBounds: Callable<F, FTag, Arg = A, Output = This<Self, B>>,
    {
        match a {
            Validity::Valid(value) => CallOnce::build(f)(value),
            Validity::Invalid(error) => Validity::Invalid(error),
        }
    }
}

// impl<A, B, F, E> Traverse<A, B, F> for ValidityFamily<E>
// where
//     F: Applicative<B, This<Self, B>>,
// {
//     fn traverse<G>(self, app: F, this: This<Self, A>, g: G) -> This<F, This<Self, B>>
//     where
//         G: Fn(A) -> This<F, B> + Copy,
//     {
//         match this {
//             Validity::Valid(value) => app.map(g(value), Validity::Valid),
//             Validity::Invalid(error) => app.pure(Validity::Invalid(error)),
//         }
//     }
// }
