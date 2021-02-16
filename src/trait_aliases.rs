pub enum MaybeOwned<'a, F> {
    Owned(F),
    Mut(&'a mut F),
    Ref(&'a F),
}

pub struct Concrete;
pub struct Generic<A>(A);

pub trait Callable<F, Tag> {
    type Arg;
    type Output;

    fn call(arg: Self::Arg, f: MaybeOwned<'_, F>) -> Self::Output;
}

impl<T: ?Sized, F: Fn(A) -> B, A, B> Callable<F, Generic<A>> for T {
    type Arg = A;
    type Output = B;

    fn call(arg: Self::Arg, f: MaybeOwned<'_, F>) -> Self::Output {
        let f = match f {
            MaybeOwned::Owned(ref f) => f,
            MaybeOwned::Mut(f) => f,
            MaybeOwned::Ref(f) => f,
        };

        f(arg)
    }
}

pub struct CallOnce<A>(A);
pub struct CallMut<A>(A);
pub struct Call<A>(A);

impl<A> CallOnce<A> {
    pub fn build<F, Tag>(f: F) -> impl FnOnce(A) -> <Self as Callable<F, Tag>>::Output
    where
        Self: Callable<F, Tag, Arg = A>,
    {
        move |a| Self::call(a, MaybeOwned::Owned(f))
    }
}

impl<A> CallMut<A> {
    pub fn build<F, Tag>(mut f: F) -> impl FnMut(A) -> <Self as Callable<F, Tag>>::Output
    where
        Self: Callable<F, Tag, Arg = A>,
    {
        move |a| Self::call(a, MaybeOwned::Mut(&mut f))
    }
}

impl<A> Call<A> {
    pub fn build<F, Tag>(f: F) -> impl Fn(A) -> <Self as Callable<F, Tag>>::Output
    where
        Self: Callable<F, Tag, Arg = A>,
    {
        move |a| Self::call(a, MaybeOwned::Ref(&f))
    }
}

impl<F: FnOnce(A) -> B, A, B> Callable<F, Concrete> for CallOnce<A> {
    type Arg = A;
    type Output = B;

    fn call(arg: Self::Arg, f: MaybeOwned<'_, F>) -> Self::Output {
        if let MaybeOwned::Owned(f) = f {
            f(arg)
        } else {
            panic!()
        }
    }
}

impl<F: FnMut(A) -> B, A, B> Callable<F, Concrete> for CallMut<A> {
    type Arg = A;
    type Output = B;

    fn call(arg: Self::Arg, f: MaybeOwned<'_, F>) -> Self::Output {
        if let MaybeOwned::Mut(f) = f {
            f(arg)
        } else {
            panic!()
        }
    }
}

impl<F: Fn(A) -> B, A, B> Callable<F, Concrete> for Call<A> {
    type Arg = A;
    type Output = B;

    fn call(arg: Self::Arg, f: MaybeOwned<'_, F>) -> Self::Output {
        if let MaybeOwned::Ref(f) = f {
            f(arg)
        } else {
            panic!()
        }
    }
}
