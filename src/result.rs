use core::{ops::{Deref, DerefMut}, fmt};

impl_shorts!(pub impl[T, E](T, E) ResultShorts => "Result": Result<T, E>
{
    fn io:is_ok(&self) -> bool;
    fn ioa:is_ok_and(self, f: impl FnOnce(Self::T) -> bool) -> bool;
    fn ie:is_err(&self) -> bool;
    fn iea:is_err_and(self, f: impl FnOnce(Self::E) -> bool) -> bool;
    fn e:err(self) -> Option<Self::E>;
    fn ar:as_ref(&self) -> Result<&Self::T, &Self::E>;
    fn am:as_mut(&mut self) -> Result<&mut Self::T, &mut Self::E>;
    fn m:map(self, op: F) -> Result<U, Self::E> [U, F: FnOnce(Self::T) -> U];
    fn mo:map_or(self, default: U, f: F) -> U [U, F: FnOnce(Self::T) -> U];
    fn moe:map_or_else(self, default: D, f: F) -> U [U, D: FnOnce(Self::E) -> U, F: FnOnce(Self::T) -> U];
    fn me:map_err(self, op: O) -> Result<Self::T, F> [F, O: FnOnce(Self::E) -> F];
    fn ip:inspect(self, f: F) -> Self [F: FnOnce(&Self::T)];
    fn ipe:inspect_err(self, f: F) -> Self [F: FnOnce(&Self::E)];
    fn ad:as_deref(&self) -> Result<&<Self::T as Deref>::Target, &Self::E> where Self::T: Deref;
    fn adm:as_deref_mut(&mut self) -> Result<&mut <Self::T as Deref>::Target, &mut Self::E> where Self::T: DerefMut;
    fn ex:expect(self, msg: &str) -> Self::T where Self::E: fmt::Debug;
    fn u:unwrap(self) -> Self::T where Self::E: fmt::Debug;
    fn uod:unwrap_or_default(self) -> Self::T where Self::T: Default;
    fn exe:expect_err(self, msg: &str) -> Self::E where Self::T: fmt::Debug;
    fn ue:unwrap_err(self) -> Self::E where Self::T: fmt::Debug;
    fn a:and(self, res: Result<U, Self::E>) -> Result<U, Self::E> [U];
    fn at:and_then(self, op: F) -> Result<U, Self::E> [U, F: FnOnce(Self::T) -> Result<U, Self::E>];
    fn o:or(self, res: Result<Self::T, F>) -> Result<Self::T, F> [F];
    fn oe:or_else(self, op: O) -> Result<Self::T, F> [F, O: FnOnce(Self::E) -> Result<Self::T, F>];
    fn uo:unwrap_or(self, default: Self::T) -> Self::T;
    fn uoe:unwrap_or_else(self, op: F) -> Self::T [F: FnOnce(Self::E) -> Self::T];
});

impl_shorts!(pub impl[T, E](T, E) ResultShortsRef => "Result": Result<&'_ T, E>
{
    fn co:copied(self) -> Result<Self::T, Self::E> where Self::T: Copy;
    fn cl:cloned(self) -> Result<Self::T, Self::E> where Self::T: Clone;
});

impl_shorts!(pub impl[T, E](T, E) ResultShortsRefMut => "Result": Result<&'_ mut T, E>
{
    fn co:copied(self) -> Result<Self::T, Self::E> where Self::T: Copy;
    fn cl:cloned(self) -> Result<Self::T, Self::E> where Self::T: Clone;
});

impl_shorts!(pub impl[T, E](T, E) ResultShortsTranspose => "Result": Result<Option<T>, E>
{
    fn tr:transpose(self) -> Option<Result<Self::T, Self::E>>;
});
