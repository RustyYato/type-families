#![feature(generic_associated_types)]
#![allow(incomplete_features)]

mod gat;
mod stable;

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

#[derive(Clone, Copy)]
pub struct OptionFamily;

use stable::*;

#[test]
fn foo() {
    let tr: Tree<u32> = tree! {
        node {
            node {
                tip = 1,
                node {
                    tip = 1,
                    tip = 2,
                }
            },
            node {
                tip = 3,
                tip = 4,
            }
        }
    };

    let x: Option<Tree<u32>> = TreeFamily.traverse(OptionFamily, tr, |x| x.checked_sub(1));

    assert_eq!(
        x,
        Some(tree! {
            node {
                node {
                    tip = 0,
                    node {
                        tip = 0,
                        tip = 1,
                    }
                },
                node {
                    tip = 2,
                    tip = 3,
                }
            }
        })
    );
}
