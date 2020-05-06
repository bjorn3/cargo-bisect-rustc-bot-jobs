#![feature(const_generics)]

trait GetType<const N:&'static str>{
    type Ty;
    fn get(&self)->&Self::Ty;
}

fn get_val<T>(value:&T)->&T::Ty
where
    T: GetType<"hello">,
{
    value.get()
}