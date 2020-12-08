use super::*;

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
