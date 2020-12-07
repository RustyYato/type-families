#![feature(generic_associated_types)]
#![allow(incomplete_features)]

mod tree;

mod gat;
mod stable;

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
