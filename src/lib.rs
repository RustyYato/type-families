#![forbid(unsafe_code)]
#![feature(generic_associated_types)]
#![allow(incomplete_features)]

pub mod tree;
pub mod validity;

// mod gat;
pub mod stable;

pub mod trait_aliases;

#[derive(Clone, Copy)]
pub struct OptionFamily;

#[test]
fn foo() {
    use self::tree::*;
    use stable::*;

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

    TreeFamily.map::<_, trait_aliases::Concrete>(tr, |x| x + 1);

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

    let x: Option<Tree<u32>> =
        TreeFamily.traverse::<_, trait_aliases::Concrete>(OptionFamily, tr, |x: u32| x.checked_sub(1));

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
