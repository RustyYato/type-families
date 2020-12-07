pub mod option;
pub mod tree;

pub type This<T, A> = <T as Family<A>>::This;
pub trait Family<A>: Copy {
    type This;
}

pub trait Functor<A, B>: Family<A> + Family<B> {
    fn map<F: Fn(A) -> B>(self, this: This<Self, A>, f: F) -> This<Self, B>;
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

#[derive(Clone, Copy)]
pub struct Iter<T>(pub T);

impl<T: Family<A>, A> Family<A> for Iter<T> {
    type This = T::This;
}

impl<Fam: Family<A> + Family<B>, F, A, B> Traverse<A, B, F> for Iter<Fam>
where
    F: Applicative<B, This<Self, B>>,
    This<Self, A>: IntoIterator<Item = A>,
    This<Self, B>: Extend<B> + Default,
{
    fn traverse<G>(self, app: F, this: This<Self, A>, g: G) -> This<F, This<Self, B>>
    where
        G: Fn(A) -> This<F, B> + Copy,
    {
        let this_b = This::<Self, B>::default();
        let mut f_this_b = app.pure(this_b);

        for f_b in this.into_iter().map(g) {
            f_this_b = app.lift_a2(f_b, f_this_b, move |b, mut this_b| {
                this_b.extend(Some(b));
                this_b
            });
        }

        f_this_b
    }
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
