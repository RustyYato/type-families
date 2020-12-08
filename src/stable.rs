pub mod iter;
pub mod option;
pub mod tree;
pub mod validity;

pub type This<T, A> = <T as Family<A>>::This;
pub trait Family<A>: Copy {
    type This;
}

pub trait Functor<A, B>: Family<A> + Family<B> {
    fn map<F: Fn(A) -> B + Copy>(self, this: This<Self, A>, f: F) -> This<Self, B>;
}

pub trait Pure<A>: Family<A> {
    fn pure(self, value: A) -> This<Self, A>;
}

pub trait Applicative<A, B>: Functor<A, B> + Pure<A> + Pure<B> {
    fn apply<F>(self, a: This<Self, F>, b: This<Self, A>) -> This<Self, B>
    where
        F: Fn(A) -> B,
        Self: Applicative<F, A>,
    {
        self.lift_a2(a, b, move |q: F, r| q(r))
    }

    fn lift_a2<C, F>(self, a: This<Self, A>, b: This<Self, B>, f: F) -> This<Self, C>
    where
        F: Fn(A, B) -> C,
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
    fn bind<F>(self, a: This<Self, A>, f: F) -> This<Self, B>
    where
        F: Fn(A) -> This<Self, B>;

    fn compose<F, G, C>(self, f: F, g: G, a: A) -> This<Self, C>
    where
        F: FnOnce(A) -> This<Self, B>,
        G: Fn(B) -> This<Self, C>,
        Self: Monad<B, C>,
    {
        self.bind(f(a), g)
    }
}

pub trait Traverse<A, B, F: Family<B> + Family<This<Self, B>>>: Family<A> + Family<B> {
    fn traverse<G>(self, app: F, this: This<Self, A>, g: G) -> This<F, This<Self, B>>
    where
        G: Fn(A) -> This<F, B> + Copy;
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
