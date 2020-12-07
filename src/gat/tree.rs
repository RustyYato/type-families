use super::{Applicative, Family, Functor, Monad, Traverse};
use crate::{Tree, TreeFamily};

impl Family for TreeFamily {
    type This<A> = Tree<A>;
}

impl Functor for TreeFamily {
    fn map<A, B, F: Fn(A) -> B>(self, this: Self::This<A>, f: F) -> Self::This<B> {
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

impl Applicative for TreeFamily {
    fn pure<A>(self, value: A) -> Self::This<A> { Tree::Tip(value) }

    fn lift_a2<A, B, C, F>(self, a: Self::This<A>, b: Self::This<B>, f: F) -> Self::This<C>
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

impl Monad for TreeFamily {
    fn bind<A, B, F>(self, a: Self::This<A>, f: F) -> Self::This<B>
    where
        F: Fn(A) -> Self::This<B>,
    {
        match a {
            Tree::Tip(value) => f(value),
            Tree::Node(a) => {
                let [l, r] = *a;
                let l: Self::This<B> = self.bind(l, &f);
                let r: Self::This<B> = self.bind(r, &f);
                Tree::Node(Box::new([l, r]))
            }
        }
    }
}

impl Traverse for TreeFamily {
    fn traverse<A, B, F: Applicative, G>(self, app: F, this: Self::This<A>, g: G) -> F::This<Self::This<B>>
    where
        G: Fn(A) -> F::This<B> + Copy,
    {
        match this {
            Tree::Tip(a) => app.map(g(a), Tree::Tip),
            Tree::Node(node) => {
                let [l, r] = *node;
                let l: F::This<Self::This<B>> = self.traverse(app, l, g);
                let r: F::This<Self::This<B>> = self.traverse(app, r, g);

                app.lift_a2(l, r, |l, r| Tree::Node(Box::new([l, r])))
            }
        }
    }
}
