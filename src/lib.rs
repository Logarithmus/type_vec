use std::marker::PhantomData;
use typenum::{consts::*, Max, Maximum, Min, Minimum};

pub struct TypeVec<Rest, Last>(PhantomData<(Rest, Last)>);

pub trait Concatenate<R> {
    type Output;
}

pub type Concat<L, R> = <L as Concatenate<R>>::Output;

// ((((), 1), 2), 3)
// ((((), 4), 5)

pub trait Same<R> {}

impl<T> Same<T> for T {}

type A = ((((((), P3), N1), N2), Z0), P10);
type C = ArrayMinimum<A>;
type D = ArrayMaximum<A>;

fn sample<T: Same<()>>() {}

fn sample2() {
    sample::<D>();
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

/// Minimum between an array and a single element
pub trait ArrayValueMin<Value> {
    type Output;
}

pub type ArrayValueMinimum<Array, Value> = <Array as ArrayValueMin<Value>>::Output;

impl<Rest, Last, Value> ArrayValueMin<Value> for (Rest, Last)
where
    Rest: ArrayValueMin<Last>,
    ArrayValueMinimum<Rest, Last>: Min<Value>,
{
    type Output = Minimum<ArrayValueMinimum<Rest, Last>, Value>;
}

impl<R> ArrayValueMin<R> for () {
    type Output = R;
}

pub trait ArrayMin {
    type Output;
}

pub type ArrayMinimum<Array> = <Array as ArrayMin>::Output;

impl<Rest: ArrayValueMin<Last>, Last> ArrayMin for (Rest, Last) {
    type Output = ArrayValueMinimum<Rest, Last>;
}

/// Minimum between an array and a single element
pub trait ArrayValueMax<Value> {
    type Output;
}

pub type ArrayValueMaximum<Array, Value> = <Array as ArrayValueMax<Value>>::Output;

impl<Rest, Last, Value> ArrayValueMax<Value> for (Rest, Last)
where
    Rest: ArrayValueMax<Last>,
    ArrayValueMaximum<Rest, Last>: Max<Value>,
{
    type Output = Maximum<ArrayValueMaximum<Rest, Last>, Value>;
}

impl<R> ArrayValueMax<R> for () {
    type Output = R;
}

pub trait ArrayMax {
    type Output;
}

pub type ArrayMaximum<Array> = <Array as ArrayMax>::Output;

impl<Rest: ArrayValueMax<Last>, Last> ArrayMax for (Rest, Last) {
    type Output = ArrayValueMaximum<Rest, Last>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
