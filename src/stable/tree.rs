use super::{Applicative, Family, Functor, Monad, Pure, This, Traverse};
use crate::{Tree, TreeFamily};

impl<A> Family<A> for TreeFamily {
    type This = Tree<A>;
}

impl<A, B> Functor<A, B> for TreeFamily {
    fn map<F: Fn(A) -> B>(self, this: This<Self, A>, f: F) -> This<Self, B> {
        match this {
            Tree::Tip(value) => Tree::Tip(f(value)),
            Tree::Node(inner) => {
                let [left, right] = *inner;
                let left = self.map(left, &f);
                let right = self.map(right, f);
                Tree::Node(Box::new([left, right]))
            }
        }
    }
}

impl<A> Pure<A> for TreeFamily {
    fn pure(self, value: A) -> This<Self, A> { Tree::Tip(value) }
}

impl<A, B, C> Applicative<A, B, C> for TreeFamily {
    fn lift_a2<F>(self, a: This<Self, A>, b: This<Self, B>, f: F) -> This<Self, C>
    where
        F: Fn(A, B) -> C,
    {
        match (a, b) {
            (Tree::Tip(a), Tree::Tip(b)) => Tree::Tip(f(a, b)),
            (Tree::Node(a), Tree::Node(b)) => {
                let [al, ar] = *a;
                let [bl, br] = *b;
                Tree::Node(Box::new([self.lift_a2(al, bl, &f), self.lift_a2(ar, br, f)]))
            }
            (a @ Tree::Tip(_), Tree::Node(b)) => {
                let [l, _] = *b;
                self.lift_a2(a, l, f)
            }
            (Tree::Node(a), b @ Tree::Tip(_)) => {
                let [_, r] = *a;
                self.lift_a2(r, b, f)
            }
        }
    }
}

impl<A, B> Monad<A, B> for TreeFamily {
    fn bind<F>(self, a: This<Self, A>, f: F) -> This<Self, B>
    where
        F: Fn(A) -> This<Self, B>,
    {
        match a {
            Tree::Tip(value) => f(value),
            Tree::Node(a) => {
                let [l, r] = *a;
                let l: This<Self, B> = self.bind(l, &f);
                let r: This<Self, B> = self.bind(r, &f);
                Tree::Node(Box::new([l, r]))
            }
        }
    }
}

impl<A, B, F> Traverse<A, B, F> for TreeFamily
where
    F: Applicative<This<Self, B>, This<Self, B>, This<Self, B>> + Functor<B, This<Self, B>>,
{
    fn traverse<G>(self, app: F, this: This<Self, A>, g: G) -> This<F, This<Self, B>>
    where
        G: Fn(A) -> This<F, B> + Copy,
    {
        match this {
            Tree::Tip(a) => app.map(g(a), Tree::Tip),
            Tree::Node(node) => {
                let [l, r] = *node;
                let l: This<F, This<Self, B>> = self.traverse(app, l, g);
                let r: This<F, This<Self, B>> = self.traverse(app, r, g);

                app.lift_a2(l, r, |l, r| Tree::Node(Box::new([l, r])))
            }
        }
    }
}
