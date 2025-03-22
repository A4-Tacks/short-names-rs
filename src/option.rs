use core::{ops::{Deref, DerefMut}, pin::Pin};

impl_shorts!(pub impl[T](T) OptionShorts => "Option": Option<T>
{
    fn is:is_some:"is_some"(&self) -> bool;
    fn isa:is_some_and:"is_some_and"(self, f: impl FnOnce(Self::T) -> bool) -> bool;
    fn ie:is_none:"is_none"(&self) -> bool;
    fn ieo:is_none_or:"is_none_or"(self, f: impl FnOnce(Self::T) -> bool) -> bool;
    fn ar:as_ref:"as_ref"(&self) -> Option<&Self::T>;
    fn am:as_mut:"as_mut"(&mut self) -> Option<&mut Self::T>;
    fn apr:as_pin_ref:"as_pin_ref"(self: Pin<&Self>) -> Option<Pin<&Self::T>>;
    fn apm:as_pin_mut:"as_pin_mut"(self: Pin<&mut Self>) -> Option<Pin<&mut Self::T>>;
    fn ai:as_slice:"as_slice"(&self) -> &[Self::T];
    fn ami:as_mut_slice:"as_mut_slice"(&mut self) -> &mut [Self::T];
    fn ex:expect:"expect"(self, msg: &str) -> Self::T;
    fn u:unwrap:"unwrap"(self) -> Self::T;
    fn uo:unwrap_or:"unwrap_or"(self, default: Self::T) -> Self::T;
    fn uoe:unwrap_or_else:"unwrap_or_else"(self, f: F) -> Self::T [F] where F: FnOnce() -> Self::T;
    fn uod:unwrap_or_default:"unwrap_or_default"(self) -> Self::T where Self::T: Default;
    fn m:map:"map"(self, f: F) -> Option<U> [U, F] where F: FnOnce(Self::T) -> U;
    fn ip:inspect:"inspect"(self, f: F) -> Self [F: FnOnce(&Self::T)];
    fn mo:map_or:"map_or"(self, default: U, f: F) -> U [U, F] where F: FnOnce(Self::T) -> U;
    fn moe:map_or_else:"map_or_else"(self, default: D, f: F) -> U [U, D, F] where D: FnOnce() -> U, F: FnOnce(Self::T) -> U;
    fn oo:ok_or:"ok_or"(self, err: E) -> Result<Self::T, E> [E];
    fn ooe:ok_or_else:"ok_or_else"(self, err: F) -> Result<Self::T, E> [E, F] where F: FnOnce() -> E;
    fn ad:as_deref:"as_deref"(&self) -> Option<&<Self::T as Deref>::Target> where Self::T: Deref;
    fn adm:as_deref_mut:"as_deref_mut"(&mut self) -> Option<&mut <Self::T as Deref>::Target> where Self::T: DerefMut;
    fn a:and:"and"(self, optb: Option<U>) -> Option<U> [U];
    fn at:and_then:"and_then"(self, f: F) -> Option<U> [U, F] where F: FnOnce(Self::T) -> Option<U>;
    fn d:filter:"filter"(self, predicate: P) -> Self [P] where P: FnOnce(&Self::T) -> bool;
    fn o:or:"or"(self, optb: Option<Self::T>) -> Option<Self::T>;
    fn oe:or_else:"or_else"(self, f: F) -> Option<Self::T> [F] where F: FnOnce() -> Option<Self::T>;
    fn x:xor:"xor"(self, optb: Option<Self::T>) -> Option<Self::T>;
    fn r:insert:"insert"(&mut self, value: Self::T) -> &mut Self::T;
    fn gor:get_or_insert:"get_or_insert"(&mut self, value: Self::T) -> &mut Self::T;
    fn gorw:get_or_insert_with:"get_or_insert_with"(&mut self, f: F) -> &mut Self::T [F] where F: FnOnce() -> Self::T;
    fn t:take:"take"(&mut self) -> Option<Self::T>;
    fn ti:take_if:"take_if"(&mut self, predicate: P) -> Option<Self::T> [P] where P: FnOnce(&mut Self::T) -> bool;
    fn rp:replace:"replace"(&mut self, value: Self::T) -> Option<Self::T>;
    fn z:zip:"zip"(self, other: Option<U>) -> Option<(Self::T, U)> [U];
});

impl_shorts!(pub impl[T, U](T, U) OptionShortsUnzip => "Option": Option<(T, U)>
{
    fn uz:unzip:"unzip"(self) -> (Option<Self::T>, Option<Self::U>);
});

impl_shorts!(pub impl[T](T) OptionShortsRef => "Option": Option<&'_ T>
{
    fn co:copied:"copied"(self) -> Option<Self::T> where Self::T: Copy;
    fn cl:cloned:"cloned"(self) -> Option<Self::T> where Self::T: Clone;
});

impl_shorts!(pub impl[T](T) OptionShortsRefMut => "Option": Option<&'_ mut T>
{
    fn co:copied:"copied"(self) -> Option<Self::T> where Self::T: Copy;
    fn cl:cloned:"cloned"(self) -> Option<Self::T> where Self::T: Clone;
});

impl_shorts!(pub impl[T, E](T, E) OptionShortsTranspose => "Option": Option<Result<T, E>>
{
    fn tr:transpose:"transpose"(self) -> Result<Option<Self::T>, Self::E>;
});
