pub mod option;
pub mod tree;

pub type This<T, A> = <T as Family<A>>::This;
pub trait Family<A>: Copy {
    type This;
}

pub trait Functor<A, B>: Family<A> + Family<B> {
    fn map<F: Fn(A) -> B>(self, this: This<Self, A>, f: F) -> This<Self, B>;
}

pub trait Applicative<A, B, C>: Functor<A, B> + Family<C> {
    fn lift_a2<F>(self, a: This<Self, A>, b: This<Self, B>, f: F) -> This<Self, C>
    where
        F: Fn(A, B) -> C;
}

pub trait Monad<A, B>: Functor<A, B> {
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

pub trait Traverse<A, B, F>: Family<A> + Family<B>
where
    F: Applicative<This<Self, B>, This<Self, B>, This<Self, B>> + Functor<B, This<Self, B>>,
{
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
