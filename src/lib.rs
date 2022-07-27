use std::marker::PhantomData;
use typenum::{Min, Minimum};

pub struct TypeVec<Rest, Last>(PhantomData<(Rest, Last)>);

pub trait Concatenate<R> {
    type Output;
}

pub type Concat<L, R> = <L as Concatenate<R>>::Output;

// ((((), 1), 2), 3)
// ((((), 4), 5)

pub trait Same<R> {}

impl<T> Same<T> for T {}

type A = (((), i8), u128);
type B = ((((), u8), u16), u32);
type C = ArrayMinimum<A, B>;

fn sample<T: Same<()>>() {}

fn sample2() {
    sample::<C>();
}

// impl<RestL, LastL, RestR, LastR> Concatenate<(RestR, LastR)> for (RestL, LastL) {
//     type Output = (Concat<(RestL, LastL), ((), RestL)>, LastR);
// }

impl<R> Concatenate<R> for () {
    type Output = R;
}

impl<RestL, LastL> Concatenate<()> for (RestL, LastL) {
    type Output = Self;
}

impl<RestL, LastL, RestR, LastR> Concatenate<(RestR, LastR)> for (RestL, LastL)
where
    (RestL, LastL): Concatenate<RestR>,
{
    type Output = (Concat<(RestL, LastL), RestR>, LastR);
}

pub trait PopTrait {
    type Output;
}

impl<L, R> PopTrait for (L, R) {
    type Output = L;
}

pub type Pop<T> = <T as PopTrait>::Output;

pub trait PushTrait<R> {
    type Output;
}

impl<L, R> PushTrait<R> for L {
    type Output = (L, R);
}

type Push<L, R> = <L as PushTrait<R>>::Output;

pub trait ArrayMin<Last = ()> {
    type Output;
}

pub type ArrayMinimum<L, R = ()> = <L as ArrayMin<R>>::Output;

impl<Rest, PreLast, Last> ArrayMin<Last> for (Rest, PreLast)
where
    Rest: ArrayMin<PreLast>,
    ArrayMinimum<Rest, PreLast>: Min<Last>,
{
    type Output = Minimum<ArrayMinimum<Rest, PreLast>, Last>;
}

impl<R> ArrayMin<R> for () {
    type Output = R;
}

#[cfg(test)]
mod tests {
    use crate::{sample, C};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
