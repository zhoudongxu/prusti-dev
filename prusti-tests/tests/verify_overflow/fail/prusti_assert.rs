// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

fn assert1() {
    prusti_assert!(true);
}

fn failing_assert() {
    prusti_assert!(false); //~ ERROR
}

fn assign() {
    let mut i = 10;
    prusti_assert!(i == 10);
    i = i * 10;
    prusti_assert!(i == 100);
    i -= 100;
    prusti_assert!(i == 2); //~ ERROR
}

fn implication(arg: i32) {
    let x;
    if arg == 0 {
        x = 1;
    } else {
        x = -1;
    }
    prusti_assert!((arg == 0 ==> x > 0) && (arg != 0 ==> x < 0));
}

//fn quantifiers() {
//    // FIXME: quantifier lowering isn't implemented with -Punsafe_core_proof
//    prusti_assert!(forall(|x: u32| x == x));
//}

fn loop_shouldnt_crash() {
    let mut i = 0;
    let mut k = 30;
    while i < 10 {
        body_invariant!(i >= 0 && i < 10);
        let old_i = i;
        i += 1;
        prusti_assert!(k > 0);
        prusti_assert!(k < 100);
        prusti_assert!(i > old_i);
    }
}

fn main() {}
