use super::{Applicative, Family, Functor, Monad, This, Traverse};
use crate::{Tree, TreeFamily};

impl<A> Family<A> for TreeFamily {
    type This = Tree<A>;
}

// impl Functor for TreeFamily {
//     fn map<A, B, F: Fn(A) -> B>(self, this: Self::This<A>, f: F) -> Self::This<B> {
//         match this {
//             Tree::Tip(value) => Tree::Tip(f(value)),
//             Tree::Node(inner) => {
//                 let [left, right] = *inner;
//                 let left = self.map(left, &f);
//                 let right = self.map(right, f);
//                 Tree::Node(Box::new([left, right]))
//             }
//         }
//     }
// }

// impl Applicative for TreeFamily {
//     fn pure<A>(self, value: A) -> Self::This<A> { Tree::Tip(value) }

//     fn lift_a2<A, B, C, F>(self, a: Self::This<A>, b: Self::This<B>, f: F) -> Self::This<C>
//     where
//         F: Fn(A, B) -> C,
//     {
//         match (a, b) {
//             (Tree::Tip(a), Tree::Tip(b)) => Tree::Tip(f(a, b)),
//             (Tree::Node(a), Tree::Node(b)) => {
//                 let [al, ar] = *a;
//                 let [bl, br] = *b;
//                 Tree::Node(Box::new([self.lift_a2(al, bl, &f), self.lift_a2(ar, br, f)]))
//             }
//             (a @ Tree::Tip(_), Tree::Node(b)) => {
//                 let [l, _] = *b;
//                 self.lift_a2(a, l, f)
//             }
//             (Tree::Node(a), b @ Tree::Tip(_)) => {
//                 let [_, r] = *a;
//                 self.lift_a2(r, b, f)
//             }
//         }
//     }
// }

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

// impl<A, B, F> Traverse<A, B, F> for TreeFamily
// where
//     F: Applicative<B, This<Self, B>, This<Self, B>>,
// {
//     fn traverse<G>(self, app: F, this: This<Self, A>, g: G) -> This<F, This<Self, B>>
//     where
//         G: Fn(A) -> This<F, B> + Copy,
//     {
//         match this {
//             Tree::Tip(a) => app.map(g(a), Tree::Tip),
//             Tree::Node(node) => {
//                 let [l, r] = *node;
//                 let l: This<F, This<Self, B>> = self.traverse(app, l, g);
//                 let r: This<F, This<Self, B>> = self.traverse(app, r, g);

//                 app.lift_a2(l, r, |l, r| Tree::Node(Box::new([l, r])))
//             }
//         }
//     }
// }
