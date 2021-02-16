use crate::trait_aliases::Callable;

pub mod iter;
pub mod option;
pub mod tree;
pub mod validity;

pub type This<T, A> = <T as Family<A>>::This;
pub trait Family<A>: Copy + Send + Sync {
    type This;
}

pub trait Functor<A, B>: Family<A> + Family<B> {
    type FuncBounds;

    fn map<F, FTag>(self, this: This<Self, A>, f: F) -> This<Self, B>
    where
        Self::FuncBounds: Callable<F, FTag, Arg = A, Output = B>;
}

pub trait Pure<A>: Family<A> {
    fn pure(self, value: A) -> This<Self, A>;
}

pub trait Applicative<A, B>: Functor<A, B> + Pure<A> + Pure<B> {
    type AppBounds; // fn(A, B) -> C

    fn lift_a2<C, F, FTag>(self, a: This<Self, A>, b: This<Self, B>, f: F) -> This<Self, C>
    where
        Self::AppBounds: Callable<F, FTag, Arg = (A, B), Output = C>,
        Self: Pure<C>;
}

#[macro_export(local_inner_macros)]
macro_rules! parse_do_notation {
    (
        $monad:expr =>
    ) => {};
    (
        $monad:expr => pure $value:expr
    ) => {
        $crate::stable::Pure::pure($monad, $value)
    };
    (
        $monad:expr => $value:expr; $($rest:tt)*
    ) => {{
        $value;
        parse_do_notation!($monad => $($rest)*)
    }};
    (
        $monad:expr => $value:stmt; $($rest:tt)*
    ) => {{
        $value
        parse_do_notation!($monad => $($rest)*)
    }};
    (
        $monad:expr =>
        $pat:pat => $value:expr;
        $($rest:tt)*
    ) => {
        $crate::stable::Monad::bind(
            $monad,
            $value,
            move |$pat| parse_do_notation!($monad => $($rest)*)
        )
    };
}

#[macro_export(local_inner_macros)]
macro_rules! do_notation {
    (
        $monad:expr => { $($expr:tt)* }
    ) => {
        parse_do_notation! {
            $monad => $($expr)*
        }
    };
}

pub trait Monad<A, B>: Applicative<A, B> {
    fn bind<F, FTag>(self, a: This<Self, A>, f: F) -> This<Self, B>
    where
        <Self as Functor<A, B>>::FuncBounds: Callable<F, FTag, Arg = A, Output = This<Self, B>>;

    fn compose<F, G, C, GTag>(self, f: F, g: G, a: A) -> This<Self, C>
    where
        F: FnOnce(A) -> This<Self, B>,
        <Self as Functor<B, C>>::FuncBounds: Callable<G, GTag, Arg = B, Output = This<Self, C>>,
        Self: Monad<B, C>,
    {
        Monad::<B, C>::bind(self, f(a), g)
    }
}

pub trait Traverse<A, B, F: Family<B> + Family<This<Self, B>>>: Family<A> + Family<B> {
    type TravBounds;

    fn traverse<G, GTag>(self, app: F, this: This<Self, A>, g: G) -> This<F, This<Self, B>>
    where
        Self::TravBounds: Callable<G, GTag, Arg = A, Output = This<F, B>>;
}

pub trait SemiGroup {
    fn append(self, rhs: Self) -> Self;
}

impl<A> SemiGroup for Vec<A> {
    fn append(mut self, rhs: Self) -> Self {
        self.extend(rhs);
        self
    }
}

impl SemiGroup for String {
    fn append(mut self, rhs: Self) -> Self {
        self.push_str(&rhs);
        self
    }
}

impl SemiGroup for () {
    fn append(self, (): Self) -> Self {}
}
