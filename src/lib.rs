pub struct HNil;
 
pub struct HCons<H, T> {
    pub head: H,
    pub tail: T
}

pub trait Contains<A> {
    fn get(&self) -> &A;
    fn get_mut(&mut self) -> &mut A;
}

macro_rules! hlist_type_internal {
    ($hlist_name:ident, $hlist_current:ty, $th:ty, $($tr:ty,)*) => {
       hlist_type_internal!($hlist_name, HCons<$th, $hlist_current>, $($tr,)*);
    };
    ($hlist_name:ident, $hlist_current:ty,) => {
        type $hlist_name = $hlist_current;
    }
}

#[macro_export]
macro_rules! hlist_type {
    ($hlist:ident) => {hlist_type_internal!($hlist, HNil,)};
    ($hlist:ident, $($types:ty),* ) => {hlist_type_internal!($hlist, $crate::HNil, $($types,)*);}
}

macro_rules! generate_hlist_contains {
    
    ($hlist:ty) => {{}};
    ($hlist:ty,) => {{}};
    
    ($hlist:ty, $last:ty, $($init:ty,)*) => {{
    
        impl $crate::Contains<$last> for $hlist {
        
            fn get(&self) -> &$last {
                let cur_cell = self;
                $(
                    let _head: &$init; // Only used to refer to $init 
                    let cur_cell = &cur_cell.tail;
                )*
                &cur_cell.head
            }
            
            fn get_mut(&mut self) -> &mut $last {
                let cur_cell = self;
                $(
                    let _head: &$init;
                    let cur_cell = &mut cur_cell.tail;
                )*
                &mut cur_cell.head
            }
        }
        
        generate_hlist_contains!($hlist, $($init,)*);
    }}
}

/// Test comment
#[macro_export]
macro_rules! generate_contains_impls {
    ($($types:ty),*) => {{
        hlist_type!(TheHList, $($types),*); 
        generate_hlist_contains!(TheHList, $($types,)*);
    }}
}

pub trait HList: Sized {
    fn push<A: Sized>(self, a: A) -> HCons<A, Self> {
        HCons { head: a, tail: self }
    }
}


 
impl HList for HNil {}
impl<H, T: HList> HList for HCons<H, T> {}


#[test]
fn hlist_type_tests() {
    hlist_type!(Zero);
    hlist_type!(One, i32);
    hlist_type!(Two, i32, i64);
    let _: Zero = HNil;
    let _: One = HNil.push(0i32);
    let _: Two = HNil.push(0i32).push(0i64);
}

#[test]
fn contains_tests() {
    generate_contains_impls!();
    generate_contains_impls!(i32);
    generate_contains_impls!(i32, i64);
    let _zero: HNil = HNil;
    let one: HCons<i32, HNil> = HNil.push(1i32);
    let two: HCons<i64, HCons<i32, HNil>> = HNil.push(1i32).push(2i64);
    let one_val: &i32 = one.get();
    let two_val_one: &i32 = two.get();
    let two_val_two: &i64 = two.get();
    assert!(one_val == &1i32);
    assert!(two_val_one == &1i32);
    assert!(two_val_two == &2i64);
}

