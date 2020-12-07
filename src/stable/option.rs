use super::{Applicative, Family, Functor, Monad, This};
use crate::OptionFamily;

impl<A> Family<A> for OptionFamily {
    type This = Option<A>;
}

impl<A, B> Functor<A, B> for OptionFamily {
    fn map<F: Fn(A) -> B>(self, this: This<Self, A>, f: F) -> This<Self, B> { this.map(f) }
}

impl<A, B, C> Applicative<A, B, C> for OptionFamily {
    fn lift_a2<F>(self, a: This<Self, A>, b: This<Self, B>, f: F) -> This<Self, C>
    where
        F: Fn(A, B) -> C,
    {
        Some(f(a?, b?))
    }
}

impl<A, B> Monad<A, B> for OptionFamily {
    fn bind<F>(self, a: This<Self, A>, f: F) -> This<Self, B>
    where
        F: Fn(A) -> This<Self, B>,
    {
        a.and_then(f)
    }
}
