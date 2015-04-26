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
        
            #[allow(unused_variables)]
            fn get(&self) -> &$last {
                let cur_cell = self;
                $(
                    let head: &$init = &cur_cell.head; // Only used to refer to $init 
                    let cur_cell = &cur_cell.tail;
                )*
                &cur_cell.head
            }
            
            #[allow(unused_variables)]
            fn get_mut(&mut self) -> &mut $last {
                let cur_cell = self;
                $(
                    let head: &$init = &cur_cell.head;
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
    fn insert<A: Sized>(self, a: A) -> HCons<A, Self> {
        HCons { head: a, tail: self }
    }
}


 
impl HList for HNil {}
impl<H, T: HList> HList for HCons<H, T> {}

#[test]
fn empty_hlist_type() {
    hlist_type!(Empty);
    let empty: Empty = HNil;
    drop(empty);
}

#[test]
fn empty_generate_contains_impls() {
    generate_contains_impls!();
}
