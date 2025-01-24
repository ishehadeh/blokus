
macro_rules! impl_bit_array_for_primitive_int {
    ($typ:ty) => {
        impl const $crate::bitboard::BitArray for $typ {
            fn new_zero(min_length: usize) -> Self {
                if min_length > <$typ>::BITS as usize {
                    panic!("min_length is too long!");
                }
        
                0 as $typ
            }
        
            fn bit_get(&self, index: usize) -> bool {
                *self & ((1 as $typ) << index) != 0
            }
        
            fn bit_set(&mut self, index: usize, value: bool) {
                if value {
                    *self |= (1 as $typ) << index;
                } else {
                    *self &= !((1 as $typ) << index);
                }
            }
        }
    };
}


impl_bit_array_for_primitive_int!(u8);
impl_bit_array_for_primitive_int!(u16);
impl_bit_array_for_primitive_int!(u32);
impl_bit_array_for_primitive_int!(u64);
impl_bit_array_for_primitive_int!(u128);