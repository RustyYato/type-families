use crate::trait_aliases::CallMut;

use super::*;

#[derive(Clone, Copy)]
pub struct Iter<T>(pub T);

impl<T: Family<A>, A> Family<A> for Iter<T> {
    type This = T::This;
}

impl<Fam: Family<A> + Family<B>, F, A, B> Traverse<A, B, F> for Iter<Fam>
where
    F: Applicative<B, This<Fam, B>>,
    This<Fam, A>: IntoIterator<Item = A>,
    This<Fam, B>: Extend<B> + Default,
{
    type TravBounds = CallMut<A>;

    fn traverse<G, GTag>(self, app: F, this: This<Self, A>, g: G) -> This<F, This<Self, B>>
    where
        Self::TravBounds: Callable<G, GTag, Arg = A, Output = This<F, B>>,
    {
        let this_b = This::<Self, B>::default();
        let mut f_this_b = app.pure(this_b);

        for f_b in this.into_iter().map(CallMut::build(g)) {
            f_this_b = app.lift_a2(f_b, f_this_b, move |(b, mut this_b)| {
                this_b.extend(Some(b));
                this_b
            });
        }

        f_this_b
    }
}
