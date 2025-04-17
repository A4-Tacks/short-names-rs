#![no_std]
#![doc = include_str!("../README.md")]

macro_rules! impl_shorts {
    ($vis:vis impl$([$($g:tt)*])?$(($($aty:ident),*))?
    $name:ident $(: $namedep:path $(: $namedeprest:path)*)?
    => $dname:literal : $ty:ty
        $(where $($wty:ty : $wbf:path $(: $wbr:path)*),+ $(,)?)?
    {$(
        fn $short:ident : $long:ident
        ($($param:tt)*)
        $(-> $retty:ty)?
        $([$($mg:tt)*])?
        $(where $($mwty:ty : $mwbf:path $(: $mwbr:path)*),+ $(,)?)?
    );* $(;)?}) => {
        $vis trait $name $(: $namedep $(+ $namedeprest)*)? {
            $($(type $aty;)*)?
            $(
                #[doc = concat!(
                    "Map to [`", $dname, "::", stringify!($long), "`]",
                )]
                #[stringify_inner::sexpr_attr(doc(alias = #stringify($long)))]
                fn $short
                $(<$($mg)*>)?
                ($($param)*) $(-> $retty)?
                $(where $($mwty: $mwbf $(+ $mwbr)*,)+)?
                ;
            )*
        }
        impl<$($($g)*)?> $name for $ty
        $(where $($wty: $wbf $(+ $wbr)*,)+)?
        {
            $($(type $aty = $aty;)*)?
            $(
                fn $short
                $(<$($mg)*>)?
                ($($param)*) $(-> $retty)?
                $(where $($mwty: $mwbf $(+ $mwbr)*,)+)?
                {
                    impl_shorts!(@impl($long => $($param)*) $($param)*)
                }
            )*
        }
    };
    (@impl($long:ident => self: $sty:ty $(, $($_t:tt)*)?) $self:ident : $sty1:ty $(, $name:ident : $ty:ty)* $(,)?) => {
        Self::$long($self $(, $name)*)
    };
    (@impl($long:ident => self $($_t:tt)*) $self:ident $(, $name:ident : $ty:ty)* $(,)?) => {
        Self::$long($self $(, $name)*)
    };
    (@impl($long:ident => &self $($_t:tt)*) &$self:ident $(, $name:ident : $ty:ty)* $(,)?) => {
        Self::$long($self $(, $name)*)
    };
    (@impl($long:ident => &mut self $($_t:tt)*) &mut $self:ident $(, $name:ident : $ty:ty)* $(,)?) => {
        Self::$long($self $(, $name)*)
    };
}

mod option;
mod result;
mod iter;

pub use option::*;
pub use result::*;
pub use iter::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = Some { 0: &mut 8 };
        let _: Option<&mut i32> = a.adm();
        let _: Option<&i32> = a.ad();
    }
}
