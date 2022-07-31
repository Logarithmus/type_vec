//! ## typelist
//! Type-level sortable singly linked list
//!
//! ### Motivation
//! The main purpose is to represent composite units in [`typeunits`](https://github.com/Logarithmus/typeunits)
//!
//! Because Rust lacks variadic generics, the list is implemented as a recursively nested tuple.
//!
//! This is similar to `typenum::TArr`, but `typelist` produces much shorter types in compilation errors:
//! ```rust
//! type List1 = (((() Const<1>), Const<2>), Const<3>)
//! type List2 = TArr<TArr<TArr<ATerm, Const<1>>, Const<2>>, Const<3>>;
//! ```
//!
//! ### Features
//! - [x] merge sort
//! - [x] minimum
//! - [x] maximum
//! - [x] concatenation
//! - [x] push
//! - [x] pop
//! - [ ] `typenum_list![..]` macro for `typenum_alias::Const<N>` list construction
//!       (TODO: fix reversed order)

use std::ops::{Add, Div};
use typenum::{
    private::{IsGreaterPrivate, PrivateIntegerAdd, PrivateIntegerAddOut},
    Bit, NonZero, PInt, Unsigned,
};
use typenum_alias::{consts::*, operator_aliases::*, type_operators::*, Const};

#[rustfmt::skip]
type A = typenum_list![5, 3, -2, 1, 2, 1, 2, 3, 4];
type B = MergeSorted<A>;

fn same<T: Same<()>>() {}

fn sample_text() {
    // this deliberately fails to compile
    // to see the type of `B` in compilation errors
    same::<A>();
    same::<B>();
}

trait Concatenate<R> {
    type Output;
}

type Concat<L, R> = <L as Concatenate<R>>::Output;

trait Same<R> {}

impl<T> Same<T> for T {}

impl<R> Concatenate<R> for () {
    type Output = R;
}

impl<RestL, LastL> Concatenate<()> for (RestL, LastL) {
    type Output = Self;
}

impl<LeftRest, LeftLast, RightRest, RightLast> Concatenate<(RightRest, RightLast)>
    for (LeftRest, LeftLast)
where
    (LeftRest, LeftLast): Concatenate<RightRest>,
{
    type Output = (Concat<(LeftRest, LeftLast), RightRest>, RightLast);
}

trait PopTrait {
    type Output;
}

impl<L, R> PopTrait for (L, R) {
    type Output = L;
}

type Pop<T> = <T as PopTrait>::Output;

trait PushTrait<R> {
    type Output;
}

impl<L, R> PushTrait<R> for L {
    type Output = (L, R);
}

type Push<L, R> = <L as PushTrait<R>>::Output;

/// Minimum between an array and a single element
trait ArrayValueMin<Value> {
    type Output;
}

type ArrayValueMinimum<Array, Value> = <Array as ArrayValueMin<Value>>::Output;

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

trait ArrayMin {
    type Output;
}

type ArrayMinimum<Array> = <Array as ArrayMin>::Output;

impl<Rest: ArrayValueMin<Last>, Last> ArrayMin for (Rest, Last) {
    type Output = ArrayValueMinimum<Rest, Last>;
}

/// Minimum between an array and a single element
trait ArrayValueMax<Value> {
    type Output;
}

type ArrayValueMaximum<Array, Value> = <Array as ArrayValueMax<Value>>::Output;

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

trait ArrayMax {
    type Output;
}

type ArrayMaximum<Array> = <Array as ArrayMax>::Output;

impl<Rest: ArrayValueMax<Last>, Last> ArrayMax for (Rest, Last) {
    type Output = ArrayValueMaximum<Rest, Last>;
}

/////////////////////////////////////////////////

trait Pair {
    type Left;
    type Right;
}

type PairL<T> = <T as Pair>::Left;
type PairR<T> = <T as Pair>::Right;

impl<L, R> Pair for (L, R) {
    type Left = L;
    type Right = R;
}

/////////////////////////////////////////////////

trait MergeSort {
    type Split: Pair;
    type Left;
    type Right;
    type Output;
}

type MergeSorted<T> = <T as MergeSort>::Output;

impl MergeSort for () {
    type Split = ((), ());
    type Left = ();
    type Right = ();
    type Output = ();
}

impl<Last> MergeSort for ((), Last) {
    type Split = (Self, ());
    type Left = Self;
    type Right = ();
    type Output = Self;
}

// <$l as Add<$r>>::Output ==>> Sum<$l, $r>

impl<Rest, PreLast, Last> MergeSort for ((Rest, PreLast), Last)
where
    Rest: Length,
    Len<Rest>: Add<P1>,
    Sum<Len<Rest>, P1>: Add<P1>,
    Sum<Sum<Len<Rest>, P1>, P1>: Div<P2>,
    ((Rest, PreLast), Last): SplitInner<(), Quot<Sum<Sum<Len<Rest>, P1>, P1>, P2>>,
    PairL<SplitHalf<((Rest, PreLast), Last)>>: MergeSort,
    PairR<SplitHalf<((Rest, PreLast), Last)>>: MergeSort,
    MergeSorted<PairL<SplitHalf<((Rest, PreLast), Last)>>>:
        Merge<MergeSorted<PairR<SplitHalf<((Rest, PreLast), Last)>>>>,
    <<((Rest, PreLast), Last) as SplitInner<
        (),
        <<Sum<Len<Rest>, P1> as Add<P1>>::Output as Div<P2>>::Output,
    >>::Output as Pair>::Left: MergeSort,
    <<((Rest, PreLast), Last) as SplitInner<
        (),
        <<<Len<Rest> as Add<P1>>::Output as Add<P1>>::Output as Div<P2>>::Output,
    >>::Output as Pair>::Right: MergeSort,
{
    type Split = SplitHalf<((Rest, PreLast), Last)>;
    type Left = PairL<Self::Split>;
    type Right = PairR<Self::Split>;
    type Output = Merged<MergeSorted<Self::Left>, MergeSorted<Self::Right>>;
}

trait Merge<Rhs> {
    type Output;
}

type Merged<L, R> = <L as Merge<R>>::Output;

impl<Rest, Last> Merge<()> for (Rest, Last) {
    type Output = Self;
}

impl<Array> Merge<Array> for () {
    type Output = Array;
}

impl<LeftRest, LeftLast, RightRest, RightLast> Merge<(RightRest, RightLast)>
    for (LeftRest, LeftLast)
where
    LeftLast: Cmp<RightLast> + IsGreaterPrivate<RightLast, Compare<LeftLast, RightLast>>,
    ((LeftRest, LeftLast), (RightRest, RightLast)):
        CmpSwap<<LeftLast as IsGreaterPrivate<RightLast, Compare<LeftLast, RightLast>>>::Output>,
{
    type Output =
        CompareSwap<((LeftRest, LeftLast), (RightRest, RightLast)), Gr<LeftLast, RightLast>>;
}

trait CmpSwap<IsGreater: Bit> {
    type Output;
}

type CompareSwap<Array, Flag> = <Array as CmpSwap<Flag>>::Output;

impl<LeftRest, LeftLast, Right> CmpSwap<B1> for ((LeftRest, LeftLast), Right)
where
    LeftRest: Merge<Right>,
{
    type Output = Push<Merged<LeftRest, Right>, LeftLast>;
}

impl<Left, RightRest, RightLast> CmpSwap<B0> for (Left, (RightRest, RightLast))
where
    Left: Merge<RightRest>,
{
    type Output = Push<Merged<Left, RightRest>, RightLast>;
}

fn merge(left: &[i8], right: &[i8]) -> Vec<i8> {
    match (left, right) {
        (left, []) => left.to_vec(),
        ([], right) => right.to_vec(),
        (left @ [left_first, left_rest @ ..], right @ [right_first, right_rest @ ..]) => {
            match left_first <= right_first {
                true => [vec![*left_first], merge(left_rest, right)].concat(),
                false => [vec![*right_first], merge(left, right_rest)].concat(),
            }
        }
    }
}

trait SplitInner<Buf, Mid> {
    type Output: Pair;
}

type Split<Array, Mid> = <Array as SplitInner<(), Diff<Len<Array>, Mid>>>::Output;

type SplitHalf<Array> = <Array as SplitInner<(), Quot<Len<Array>, P2>>>::Output;

impl<Array, Buf: Inverse<()>> SplitInner<Buf, Z0> for Array {
    type Output = (Array, Mirror<Buf>);
}

impl<Rest, Last, Buf, Mid> SplitInner<Buf, PInt<Mid>> for (Rest, Last)
where
    Mid: NonZero + Unsigned + Cmp<U1> + PrivateIntegerAdd<Compare<Mid, U1>, U1>,
    Rest: SplitInner<(Buf, Last), PrivateIntegerAddOut<Mid, Compare<Mid, U1>, U1>>,
{
    type Output = <Rest as SplitInner<Push<Buf, Last>, Diff<PInt<Mid>, P1>>>::Output;
}

trait Inverse<Buf> {
    type Output;
}

type Mirror<Array> = <Array as Inverse<()>>::Output;

impl<Buf> Inverse<Buf> for () {
    type Output = Buf;
}

impl<Rest: Inverse<(Buf, Last)>, Last, Buf> Inverse<Buf> for (Rest, Last) {
    type Output = <Rest as Inverse<Push<Buf, Last>>>::Output;
}

trait Length {
    type Output;
}

type Len<Array> = <Array as Length>::Output;

impl Length for () {
    type Output = Z0;
}

impl<Rest: Length, Last> Length for (Rest, Last)
where
    Len<Rest>: Add<P1>,
{
    type Output = Sum<Len<Rest>, P1>;
}

#[macro_export]
macro_rules! typelist {
    () => {
        ()
    };
    ($n:ty) => {
        ((), $n)
    };
    ($n:ty,) => {
        ((), $n)
    };
    ($n:ty, $($tail:ty),+) => {
        (typelist![$($tail),+], $n)
    };
    ($n:ty, $($tail:ty),+,) => {
        (typelist![$($tail),+], $n)
    };
}

#[macro_export]
macro_rules! typenum_list {
    ($($num:literal),+) => {
        apply_args_reverse!(typenum_list_inner, $($num),+)
    };
}

macro_rules! typenum_list_inner {
    ($($num:literal),+) => {
        typelist![$(Const<$num>),+]
    };
}

macro_rules! apply_args_reverse {
    ($macro_id:tt [] $($reversed:tt)*) => {
        $macro_id!($($reversed) *)
    };
    ($macro_id:tt [$first:tt $($rest:tt)*] $($reversed:tt)*) => {
        apply_args_reverse!($macro_id [$($rest)*] $first $($reversed)*)
    };
    // Entry point, use brackets to recursively reverse above.
    ($macro_id:tt, $($t:tt)*) => {
        apply_args_reverse!($macro_id [ $($t)* ])
    };
}

use apply_args_reverse;
use typenum_list_inner;

// The functions below are written in recursively-functional and immutable way
// to model the properties and restrictions of type-level computations in Rust
#[cfg(test)]
mod tests {
    fn merge(left: &[i8], right: &[i8]) -> Vec<i8> {
        match (left, right) {
            (left, []) => left.to_vec(),
            ([], right) => right.to_vec(),
            (left @ [left_first, left_rest @ ..], right @ [right_first, right_rest @ ..]) => {
                match left_first <= right_first {
                    true => [vec![*left_first], merge(left_rest, right)].concat(),
                    false => [vec![*right_first], merge(left, right_rest)].concat(),
                }
            }
        }
    }

    fn merge_sort(slice: &[i8]) -> Vec<i8> {
        match slice {
            [] => vec![],
            [el] => vec![*el],
            slice => {
                let (left, right) = slice.split_at(slice.len() / 2);
                merge(&merge_sort(left), &merge_sort(right))
            }
        }
    }

    fn split_inner(rest: &[i8], slice: &[i8], i: usize) -> (Vec<i8>, Vec<i8>) {
        match (rest, slice, i) {
            (rest, slice, 0) => (rest.to_vec(), slice.to_vec()),
            (_rest, &[], _i) => panic!("Bad index"),
            (rest, [first, tail @ ..], i) => split_inner(&[rest, &[*first]].concat(), tail, i - 1),
        }
    }

    fn split(slice: &[i8], i: usize) -> (Vec<i8>, Vec<i8>) {
        split_inner(&[], slice, i)
    }

    #[test]
    fn split_test() {
        const SLICE: &[i8] = &[1, 2, 3, 4, 5, 4, 1, 0];
        const INDEX: usize = 4;
        let s = split(SLICE, INDEX);
        assert_eq!((&s.0[..], &s.1[..]), SLICE.split_at(INDEX))
    }

    #[test]
    fn merge_sort_test() {
        assert_eq!(merge_sort(&[5, 100, 3, 10, -1]), &[-1, 3, 5, 10, 100]);
        assert_eq!(
            merge_sort(&[50, -100, 5, 100, 3, 10, -1, 100]),
            &[-100, -1, 3, 5, 10, 50, 100, 100]
        );
    }
}
