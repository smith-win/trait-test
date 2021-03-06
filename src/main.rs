


// The Write-up:
// The OCI api when used in batch mode, needs all values for a parameter to be of the same type.
// So SQLT_CHR, SQLT_NUM etc.  We want our API to ensure that this is the case, so that
// we dont encode values in the buffer in the wrong way.
// We also want some flexibility, so we can use different number tpyes (u8, i32, float) to write
// to number columns etc, and also provide an extensible mechanism, e.g we can use a different
// Decimal number library, Date/Time library without having to change the internals.

// I also wanted to to avoid dynamic dispatch.  A dynamic dispatch approach may have simpler
// to implement.   The performance impact of dynamic dispatch may be negligible when you
// consider the time taken in network and Oracle - but this is learning exercise for me.

// Hence a macro based approach is the one chosen.
// If, we want to use a function without self/&self, an non-instance method
// we will need to provide the type to the macro, so it can write out the 
// explicit code needed, like "AsOciNum::capped_size()"


/// This macro generates the trait.   This ensures that all type support traits
/// have the same methods
macro_rules! sqlt_trait{
    ($trait_name:ident, $sqlt:expr, $capped_size:expr) => {
        #[doc="$sqlt"]
        pub trait $trait_name {
            /// Sets the OCI SQLT_xxx type represented by this trait
            fn oci_sqlt() -> u16 {
                $sqlt
            }

            /// Implementors use to write the bytes in accordance with spec for $sqlt
            fn oci_write(&self, _slice: &mut [u8]) -> u16 ;

            /// Non-instance method - to get fixed size for the type (if needed)
            fn oci_capped_size() -> Option<u16> {
                $capped_size
            }
        }
    }
}

// Some dummy vaues
const SQLT_CHR:u16 = 1234;
const SQLT_INT:u16 = 1235;
const SQLT_NUM:u16 = 1235;

sqlt_trait!(AsOciChr, SQLT_CHR, None);
sqlt_trait!(AsOciInt, SQLT_INT, Some(8));
sqlt_trait!(AsOciNum, SQLT_NUM, Some(21));

// -- -- sample imples for i32
impl AsOciInt for i32 {
    fn oci_write(&self, slice: &mut [u8]) -> u16 {
        let x  = self.to_ne_bytes();
        slice[0] = x[0];
        slice[1] = x[1];
        slice[2] = x[2];
        slice[3] = x[3];
        std::mem::size_of::<i32>() as u16
    }

    // fn oci_capped_size() -> Option<u16> {
    //     Some(4u16)
    // }
}

impl AsOciInt for i16 {
    fn oci_write(&self, slice: &mut [u8]) -> u16 {
        let x  = self.to_ne_bytes();
        slice[0] = x[0];
        slice[1] = x[1];
        2
    }
    fn oci_capped_size() -> Option<u16> {
        Some(2u16)
    }
}

impl AsOciNum for i32 {
    fn oci_write(&self, _slice: &mut [u8]) -> u16 {
        todo!()
    }

    // The capped size should be in the trait!
    fn oci_capped_size() -> Option<u16> {
        Some(21u16)
    }
}



fn main() {
    println!("Hello, world!");
    param::set_param_int(1, 10i32);
    param::set_param_int(1, 10i16);
    // set_param_int(1, 10i8); // it no worky-worky
    param::set_param_num(1, 10i32);
}

/// For managing param setting
pub mod param {

    use super::*;
    use std::vec::Vec;


    /// This macro implements the 
    macro_rules! internal_buffer_set {
        ($col:expr, $to_oci:expr) => {

            let mut v = Vec::with_capacity(10);
            v.resize(10, 0);
            let result = $to_oci.oci_write(v.as_mut_slice());
            println!("Set column {}, size {:?}, capped: {:?}", $col, result, T::oci_capped_size());

        };
    }

    pub fn set_param_num<T: AsOciNum>(col: u16, t: T) {
        internal_buffer_set!(col, t);
    }
    
    pub fn set_param_int<T: AsOciInt>(col: u16, t: T) {
        internal_buffer_set!(col, t);
    }


}


