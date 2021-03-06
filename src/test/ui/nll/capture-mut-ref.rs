// Check that capturing a mutable reference by move and assigning to its
// referent doesn't make the unused mut lint think that it is mutable.

#![feature(nll)]
#![deny(unused_mut)]

fn mutable_upvar() {
    let mut x = &mut 0;
    //~^ ERROR
    move || {
        *x = 1;
    };
}

fn main() {}
