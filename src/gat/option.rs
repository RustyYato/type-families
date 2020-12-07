use super::{Applicative, Family, Functor, Monad, Traverse};
use crate::OptionFamily;

impl Family for OptionFamily {
    type This<A> = Option<A>;
}

impl Functor for OptionFamily {
    fn map<A, B, F: Fn(A) -> B>(self, this: Self::This<A>, f: F) -> Self::This<B> { this.map(f) }
}

impl Applicative for OptionFamily {
    fn pure<A>(self, value: A) -> Self::This<A> { Some(value) }

    fn lift_a2<A, B, C, F>(self, a: Self::This<A>, b: Self::This<B>, f: F) -> Self::This<C>
    where
        F: Fn(A, B) -> C,
    {
        Some(f(a?, b?))
    }
}

impl Monad for OptionFamily {
    fn bind<A, B, F>(self, a: Self::This<A>, f: F) -> Self::This<B>
    where
        F: Fn(A) -> Self::This<B>,
    {
        a.and_then(f)
    }
}

impl Traverse for OptionFamily {
    fn traverse<A, B, F: Applicative, G>(self, app: F, this: Self::This<A>, g: G) -> F::This<Self::This<B>>
    where
        G: Fn(A) -> F::This<B>,
    {
        match this {
            None => app.pure(None),
            Some(x) => app.map(g(x), Some),
        }
    }
}
