pub mod option;
pub mod tree;

pub trait Family: Copy {
    type This<A>;
}

pub trait Functor: Family {
    fn map<A, B, F: Fn(A) -> B>(self, this: Self::This<A>, f: F) -> Self::This<B>;
}

pub trait Applicative: Functor {
    fn pure<A>(self, value: A) -> Self::This<A>;

    fn apply<A, B, F>(self, a: Self::This<F>, b: Self::This<A>) -> Self::This<B>
    where
        F: Fn(A) -> B,
    {
        self.lift_a2(a, b, move |q, r| q(r))
    }

    fn lift_a2<A, B, C, F>(self, a: Self::This<A>, b: Self::This<B>, f: F) -> Self::This<C>
    where
        F: Fn(A, B) -> C;
}

pub trait Monad: Applicative {
    fn bind<A, B, F>(self, a: Self::This<A>, f: F) -> Self::This<B>
    where
        F: Fn(A) -> Self::This<B>;

    fn compose<A, B, C, F, G>(self, f: F, g: G, a: A) -> Self::This<C>
    where
        F: FnOnce(A) -> Self::This<B>,
        G: Fn(B) -> Self::This<C>,
    {
        self.bind(f(a), g)
    }

    fn join<A>(self, outer: Self::This<Self::This<A>>) -> Self::This<A> { self.bind(outer, core::convert::identity) }
}

pub trait Traverse: Family {
    fn traverse<A, B, F: Applicative, G>(self, app: F, this: Self::This<A>, g: G) -> F::This<Self::This<B>>
    where
        G: Fn(A) -> F::This<B> + Copy;
}
