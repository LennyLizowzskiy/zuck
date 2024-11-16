#[inline(always)]
pub fn should_apply_plural(input: u64) -> bool {
    input != 1
}

macro_rules! define_checked_int {
    ($mod_name:ident, $struct_name:ident, $int_type:ty) => {
        pub mod $mod_name {
            use core::ops::Deref;

            /// `Option<$int_type>` wrapper for calculations with overflow checks and less verbose code
            #[derive(Debug)]
            pub struct $struct_name(pub Option<$int_type>);

            impl From<$int_type> for $struct_name {
                fn from(value: $int_type) -> Self {
                    $struct_name(Some(value))
                }
            }

            impl Into<$struct_name> for Option<$int_type> {
                fn into(self) -> $struct_name {
                    $struct_name(self)
                }
            }

            impl Deref for $struct_name {
                type Target = Option<$int_type>;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl $struct_name {
                /// Calculates `self + (lhs * rhs)` with overflow check
                pub fn add_mul_result(self, lhs: $int_type, rhs: $int_type) -> $struct_name {
                    fn wrapper(
                        s: $struct_name,
                        lhs: $int_type,
                        rhs: $int_type,
                    ) -> Option<$int_type> {
                        let s = s.0?;

                        s.checked_add(lhs.checked_mul(rhs)?)
                    }

                    $struct_name(wrapper(self, lhs, rhs))
                }
            }
        }
    };
}

define_checked_int!(checkedu64, CheckedU64, u64);
define_checked_int!(checkedu128, CheckedU128, u128);
