use super::{Applicative, Family, Functor, Monad, Traverse};

#[macro_export(local_inner_macros)]
macro_rules! node {
    (@first node { $($inner:tt)* }, $($rest:tt)*) => {
        node!{
            @second [tree!(node { $($inner)* })] $($rest)*
        }
    };
    (@first tip = $value:expr, $($rest:tt)*) => {
        node!{
            @second [Tree::Tip($value)] $($rest)*
        }
    };
    (@second [$first:expr] node { $($inner:tt)* } $(,)?) => {
        Tree::Node(Box::new([
            $first,
            tree!(node { $($inner)* }),
        ]))
    };
    (@second [$first:expr] tip = $value:expr $(,)?) => {
        Tree::Node(Box::new([
            $first,
            Tree::Tip($value),
        ]))
    };
}

#[macro_export(local_inner_macros)]
macro_rules! tree {
    (node { $($inner:tt)* }) => {
        node!(@first $($inner)*)
    };
    (tip = $value:expr) => {
        Tree::Tip($value)
    };
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tree<A> {
    Tip(A),
    Node(Box<[Tree<A>; 2]>),
}

#[derive(Clone, Copy)]
pub struct TreeFamily;

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

// impl Monad for TreeFamily {
//     fn bind<A, B, F>(self, a: Self::This<A>, f: F) -> Self::This<B>
//     where
//         F: Fn(A) -> Self::This<B>,
//     {
//         match a {
//             Tree::Empty => Tree::Empty,
//             Tree::Node(a) => {
//                 let (r, v, l) = *a;
//                 let r: Self::This<B> = self.bind(r, &f);
//                 let v: Self::This<B> = f(v);
//                 let l: Self::This<B> = self.bind(l, &f);
//                 Tree::Empty
//             }
//         }
//     }
// }

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
