//! Generic utility macros that don't belong to an obvious module.

use macro_pub::macro_pub;

/// Simple utility macro that defines and instantiates a structure at one
/// moment. Creating the structure is a combbination of definition and
/// initialization syntax.
///
/// There are a few things to note:
///
/// - Do not use the `struct` keyword in front of the name.
/// - Visibility of the structure is always private; do not use `pub` or
///   `pub(...)` in front of the name.
/// - The name of the structure serves no purpose other than as input to
///   meta-attributes such as derives or attribute macros. For example, an
///   implimentation of [`std::fmt::Debug`].
/// - Because this declares *and* instantiates, the fields follow the same
///   syntax as a variable declaration. The types of the fields cannot be
///   elided.
/// - Field names can pe prefixed with a visibility qualifier, same as a
///   structure definition.
/// - If lifetimes are used in field types, they must be included in angle
///   brackets after the structure name, same as a declaration.
#[macro_pub]
macro_rules! new_struct {
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
