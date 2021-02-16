use super::{Applicative, Family, Functor, Monad, Pure, This};
use crate::{
    trait_aliases::{CallOnce, Callable},
    OptionFamily,
};

impl<A> Family<A> for OptionFamily {
    type This = Option<A>;
}

impl<A, B> Functor<A, B> for OptionFamily {
    type FuncBounds = CallOnce<A>;

    fn map<F, FTag>(self, this: This<Self, A>, f: F) -> This<Self, B>
    where
        Self::FuncBounds: Callable<F, FTag, Arg = A, Output = B>,
    {
        this.map(CallOnce::build(f))
    }
}

impl<A> Pure<A> for OptionFamily {
    fn pure(self, value: A) -> This<Self, A> { Some(value) }
}

impl<A, B> Applicative<A, B> for OptionFamily {
    type AppBounds = CallOnce<(A, B)>;

    fn lift_a2<C, F, FTag>(self, a: This<Self, A>, b: This<Self, B>, f: F) -> This<Self, C>
    where
        Self::AppBounds: Callable<F, FTag, Arg = (A, B), Output = C>,
    {
        Some(CallOnce::build(f)((a?, b?)))
    }
}

impl<A, B> Monad<A, B> for OptionFamily {
    fn bind<F, FTag>(self, a: This<Self, A>, f: F) -> This<Self, B>
    where
        <Self as Functor<A, B>>::FuncBounds: Callable<F, FTag, Arg = A, Output = This<Self, B>>,
    {
        a.and_then(CallOnce::build(f))
    }
}
