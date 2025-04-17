use core::{iter::*, cmp::Ordering};

impl_shorts!(pub impl[Iter: Iterator] IteratorShorts: Iterator:Sized => "Iterator": Iter
{
    fn n:next(&mut self) -> Option<Self::Item>;
    fn sh:size_hint(&self) -> (usize, Option<usize>);
    fn ct:count(self) -> usize;
    fn l:last(self) -> Option<Self::Item>;
    fn sb:step_by(self, step: usize) -> StepBy<Self>;
    fn c:chain(self, other: U) -> Chain<Self, U::IntoIter> [U] where U: IntoIterator<Item = Self::Item>;
    fn z:zip(self, other: U) -> Zip<Self, U::IntoIter> [U] where U: IntoIterator;
    fn m:map(self, f: F) -> Map<Self, F> [B, F] where F: FnMut(Self::Item) -> B;
    fn fe:for_each(self, f: F) [F] where F: FnMut(Self::Item);
    fn f:filter(self, predicate: P) -> Filter<Self, P> [P] where P: FnMut(&Self::Item) -> bool;
    fn fm:filter_map(self, f: F) -> FilterMap<Self, F> [B, F] where F: FnMut(Self::Item) -> Option<B>;
    fn e:enumerate(self) -> Enumerate<Self>;
    fn pe:peekable(self) -> Peekable<Self>;
    fn sw:skip_while(self, predicate: P) -> SkipWhile<Self, P> [P] where P: FnMut(&Self::Item) -> bool;
    fn tw:take_while(self, predicate: P) -> TakeWhile<Self, P> [P] where P: FnMut(&Self::Item) -> bool;
    fn mw:map_while(self, predicate: P) -> MapWhile<Self, P> [B, P] where P: FnMut(Self::Item) -> Option<B>;
    fn s:skip(self, n: usize) -> Skip<Self>;
    fn t:take(self, n: usize) -> Take<Self>;
    fn sc:scan(self, initial_state: St, f: F) -> Scan<Self, St, F> [St, B, F] where F: FnMut(&mut St, Self::Item) -> Option<B>;
    fn flm:flat_map(self, f: F) -> FlatMap<Self, U, F> [U, F] where U: IntoIterator, F: FnMut(Self::Item) -> U;
    fn fl:flatten(self) -> Flatten<Self> where Self::Item: IntoIterator;
    fn fu:fuse(self) -> Fuse<Self>;
    fn ip:inspect(self, f: F) -> Inspect<Self, F> [F] where F: FnMut(&Self::Item);
    fn by:by_ref(&mut self) -> &mut Self;
    fn col:collect(self) -> B [B: FromIterator<Self::Item>];
    fn pa:partition(self, f: F) -> (B, B) [B, F] where B: Default : Extend<Self::Item>, F: FnMut(&Self::Item) -> bool;
    fn fo:fold(self, init: B, f: F) -> B [B, F] where F: FnMut(B, Self::Item) -> B;
    fn re:reduce(self, f: F) -> Option<Self::Item> [F] where F: FnMut(Self::Item, Self::Item) -> Self::Item;
    fn fi:find(&mut self, predicate: P) -> Option<Self::Item> [P] where P: FnMut(&Self::Item) -> bool;
    fn fim:find_map(&mut self, f: F) -> Option<B> [B, F] where F: FnMut(Self::Item) -> Option<B>;
    fn po:position(&mut self, predicate: P) -> Option<usize> [P] where P: FnMut(Self::Item) -> bool;
    fn rpo:rposition(&mut self, predicate: P) -> Option<usize> [P] where P: FnMut(Self::Item) -> bool, Self: ExactSizeIterator : DoubleEndedIterator;
    fn maxbk:max_by_key(self, f: F) -> Option<Self::Item> [B: Ord, F] where F: FnMut(&Self::Item) -> B;
    fn maxb:max_by(self, compare: F) -> Option<Self::Item> [F] where F: FnMut(&Self::Item, &Self::Item) -> Ordering;
    fn minbk:min_by_key(self, f: F) -> Option<Self::Item> [B: Ord, F] where F: FnMut(&Self::Item) -> B;
    fn minb:min_by(self, compare: F) -> Option<Self::Item> [F] where F: FnMut(&Self::Item, &Self::Item) -> Ordering;
    fn r:rev(self) -> Rev<Self> where Self: DoubleEndedIterator;
    fn u:unzip(self) -> (FromA, FromB) [A, B, FromA, FromB] where FromA: Default : Extend<A>, FromB: Default : Extend<B>, Self: Iterator<Item = (A, B)>;
    fn co:copied(self) -> Copied<Self> ['a, T: 'a] where Self: Iterator<Item = &'a T>, T: Copy;
    fn cl:cloned(self) -> Cloned<Self> ['a, T: 'a] where Self: Iterator<Item = &'a T>, T: Clone;
    fn cy:cycle(self) -> Cycle<Self> where Self: Clone;
    fn prod:product(self) -> P [P] where P: Product<Self::Item>;
    fn pcmp:partial_cmp(self, other: I) -> Option<Ordering> [I] where I: IntoIterator, Self::Item: PartialOrd<I::Item>;
});

impl_shorts!(pub impl[Iter: DoubleEndedIterator] DoubleEndedIteratorShorts: DoubleEndedIterator:Sized => "DoubleEndedIterator": Iter
{
    fn nb:next_back(&mut self) -> Option<Self::Item>;
    fn nthb:nth_back(&mut self, n: usize) -> Option<Self::Item>;
    fn rfi:rfind(&mut self, predicate: P) -> Option<Self::Item> [P] where P: FnMut(&Self::Item) -> bool;
    fn rfo:rfold(self, init: B, f: F) -> B [B, F] where F: FnMut(B, Self::Item) -> B;
});
