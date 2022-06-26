#[doc(hidden)]
#[macro_export]
macro_rules! new_struct_impl {
    (
        $(#[$struct_meta:meta])*
        $struct_name:ident $(<$($struct_life:lifetime),+>)? {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field_name:ident: $field_type:ty = $field_value:expr,
            )+
        }
    ) => {{
        $(#[$struct_meta])*
        struct $struct_name $(<$($struct_life),*>)? {
            $(
                $(#[$field_meta])*
                $field_vis $field_name: $field_type,
            )*
        }

        $struct_name {
            $($field_name: $field_value,)*
        }
    }};
}

pub use new_struct_impl as new_struct;
