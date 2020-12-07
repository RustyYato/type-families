#[macro_export(local_inner_macros)]
macro_rules! node {
    (@first node { $($inner:tt)* }, $($rest:tt)*) => {
        node!{
            @second [tree!(node { $($inner)* })] $($rest)*
        }
    };
    (@first tip = $value:expr, $($rest:tt)*) => {
        node!{
            @second [$crate::tree::Tree::Tip($value)] $($rest)*
        }
    };
    (@second [$first:expr] node { $($inner:tt)* } $(,)?) => {
        $crate::tree::Tree::Node(Box::new([
            $first,
            tree!(node { $($inner)* }),
        ]))
    };
    (@second [$first:expr] tip = $value:expr $(,)?) => {
        $crate::tree::Tree::Node(Box::new([
            $first,
            $crate::tree::Tree::Tip($value),
        ]))
    };
}

#[macro_export(local_inner_macros)]
macro_rules! tree {
    (node { $($inner:tt)* }) => {
        node!(@first $($inner)*)
    };
    (tip = $value:expr) => {
        $crate::tree::Tree::Tip($value)
    };
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tree<A> {
    Tip(A),
    Node(Box<[Tree<A>; 2]>),
}

#[derive(Clone, Copy)]
pub struct TreeFamily;
