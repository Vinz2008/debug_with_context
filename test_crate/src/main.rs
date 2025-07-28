#![feature(debug_closure_helpers)]

use debug_with_context::{DebugWithContext, DebugWrapContext};
use std::fmt::Debug;

struct Context;

#[derive(DebugWithContext)]
#[debug_context(Context)]
struct Test {
    a : i32,
    b : u64,
    c : usize,
    d : String,
}

#[derive(DebugWithContext)]
#[debug_context(Context)]
struct TestGenerics<T, A>
where A: Debug
{
    a : T,
    b : Vec<A>,
}

#[derive(DebugWithContext)]
#[debug_context(Context)]
enum TestE {
    VariantA { a: i32 },
    VariantB { b: i64 },
    VariantC { c: String },
    VariantD { d: Vec<u64> }
}

#[derive(DebugWithContext)]
#[debug_context(Context)]
struct TestT(u64);

#[derive(DebugWithContext)]
#[debug_context(Context)]
enum TestET {
    VariantA(i32),
    VariantB(u64),
}

#[derive(DebugWithContext)]
#[debug_context(Context)]
enum TestEBoth {
    VariantA(i32),
    VariantB { b: i64 },
}

fn main(){
    let context = Context;
    let test = Test {
        a: 3,
        b : 6,
        c : 7,
        d: "oo".to_string(),
    };

    println!("{:?}", DebugWrapContext::new(&test, &context));

    let test = TestT(6);
    println!("{:?}", DebugWrapContext::new(&test, &context));

    let teste = TestE::VariantA {a: 2 };
    println!("{:?}", DebugWrapContext::new(&teste, &context));
    let teste = TestE::VariantB { b: 3 };
    println!("{:?}", DebugWrapContext::new(&teste, &context));

    let teste = TestE::VariantA {a: 2 };
    println!("{:?}", DebugWrapContext::new(&teste, &context));
    let teste = TestE::VariantB { b: 3 };
    println!("{:?}", DebugWrapContext::new(&teste, &context));

    let testet = TestET::VariantA(2);
    println!("{:?}", DebugWrapContext::new(&testet, &context));
    let testet = TestET::VariantB(3);
    println!("{:?}", DebugWrapContext::new(&testet, &context));

    let testeboth = TestEBoth::VariantA(2);
    println!("{:?}", DebugWrapContext::new(&testeboth, &context));
    let testeboth = TestEBoth::VariantB { b: 3 };
    println!("{:?}", DebugWrapContext::new(&testeboth, &context));
}