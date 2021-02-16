use super::{Applicative, Family, Functor, Monad, Pure, This, Traverse};
use crate::{
    trait_aliases::{CallMut, Callable},
    tree::{Tree, TreeFamily},
};

impl<A> Family<A> for TreeFamily {
    type This = Tree<A>;
}

impl TreeFamily {
    fn map_helper<A, B, F>(self, this: This<Self, A>, f: &mut F) -> This<Self, B>
    where
        F: FnMut(A) -> B,
    {
        match this {
            Tree::Tip(value) => Tree::Tip(f(value)),
            Tree::Node(inner) => {
                let [left, right] = *inner;
                let left = self.map_helper(left, f);
                let right = self.map_helper(right, f);
                Tree::Node(Box::new([left, right]))
            }
        }
    }

    fn lift_a2_helper<A, B, C, F>(self, a: This<Self, A>, b: This<Self, B>, f: &mut F) -> This<Self, C>
    where
        F: FnMut((A, B)) -> C,
    {
        match (a, b) {
            (Tree::Tip(a), Tree::Tip(b)) => Tree::Tip(f((a, b))),
            (Tree::Node(a), Tree::Node(b)) => {
                let [al, ar] = *a;
                let [bl, br] = *b;
                Tree::Node(Box::new([
                    self.lift_a2_helper(al, bl, f),
                    self.lift_a2_helper(ar, br, f),
                ]))
            }
            (a @ Tree::Tip(_), Tree::Node(b)) => {
                let [l, _] = *b;
                self.lift_a2_helper(a, l, f)
            }
            (Tree::Node(a), b @ Tree::Tip(_)) => {
                let [_, r] = *a;
                self.lift_a2_helper(r, b, f)
            }
        }
    }

    fn bind_helper<A, B, F>(self, a: This<Self, A>, f: &mut F) -> This<Self, B>
    where
        F: FnMut(A) -> This<Self, B>,
    {
        match a {
            Tree::Tip(value) => f(value),
            Tree::Node(a) => {
                let [l, r] = *a;
                let l: This<Self, B> = self.bind_helper(l, f);
                let r: This<Self, B> = self.bind_helper(r, f);
                Tree::Node(Box::new([l, r]))
            }
        }
    }

    fn traverse_helper<A, B, F, G>(self, app: F, this: This<Self, A>, g: &mut G) -> This<F, This<Self, B>>
    where
        F: Applicative<Tree<B>, Tree<B>> + Functor<B, Tree<B>>,
        G: FnMut(A) -> This<F, B>,
    {
        match this {
            Tree::Tip(a) => Functor::<B, _>::map(app, g(a), Tree::Tip),
            Tree::Node(node) => {
                let [l, r] = *node;
                let l: This<F, This<Self, B>> = self.traverse_helper(app, l, g);
                let r: This<F, This<Self, B>> = self.traverse_helper(app, r, g);

                app.lift_a2(l, r, |(l, r)| Tree::Node(Box::new([l, r])))
            }
        }
    }
}

impl<A, B> Functor<A, B> for TreeFamily {
    type FuncBounds = CallMut<A>;

    fn map<F, FTag>(self, this: This<Self, A>, f: F) -> This<Self, B>
    where
        Self::FuncBounds: Callable<F, FTag, Arg = A, Output = B>,
    {
        self.map_helper(this, &mut CallMut::build(f))
    }
}

impl<A> Pure<A> for TreeFamily {
    fn pure(self, value: A) -> This<Self, A> { Tree::Tip(value) }
}

impl<A, B> Applicative<A, B> for TreeFamily {
    type AppBounds = CallMut<(A, B)>;

    fn lift_a2<C, F, FTag>(self, a: This<Self, A>, b: This<Self, B>, f: F) -> This<Self, C>
    where
        Self::AppBounds: Callable<F, FTag, Arg = (A, B), Output = C>,
    {
        self.lift_a2_helper(a, b, &mut CallMut::build(f))
    }
}

impl<A, B> Monad<A, B> for TreeFamily {
    fn bind<F, FTag>(self, a: This<Self, A>, f: F) -> This<Self, B>
    where
        Self::FuncBounds: Callable<F, FTag, Arg = A, Output = This<Self, B>>,
    {
        self.bind_helper(a, &mut CallMut::build(f))
    }
}

impl<A, B, F> Traverse<A, B, F> for TreeFamily
where
    F: Applicative<Tree<B>, Tree<B>> + Functor<B, Tree<B>>,
{
    type TravBounds = CallMut<A>;

    fn traverse<G, GTag>(self, app: F, this: This<Self, A>, g: G) -> This<F, This<Self, B>>
    where
        Self::TravBounds: Callable<G, GTag, Arg = A, Output = This<F, B>>,
    {
        self.traverse_helper(app, this, &mut CallMut::build(g))
    }
}
