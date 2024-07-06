pub mod auto_racing;
pub mod baseball;
pub mod basketball;
pub mod cricket;
pub mod event_counter;
pub mod football;
pub mod hockey_lacrosse;
pub mod judo;
pub mod karate;
pub mod lane_timer;
pub mod pitch_and_speed;
pub mod rodeo;
pub mod soccer;
pub mod strike_out_count;
pub mod taekwondo;
pub mod tennis;
pub mod track;
pub mod volleyball;
pub mod water_polo;
pub mod wrestling;

/// The trait for all sports to implement.
pub trait Sport<DS: crate::rtd_state::data_source::RTDStateDataSource> {
    /// Get the name of the sport.
    fn name(&self) -> &'static str;

    /// Get the backing RTDState.
    fn rtd_state(&mut self) -> &mut crate::rtd_state::RTDState<DS>;
}

#[doc(hidden)]
pub mod macros {
    /// Builds a sport
    ///
    /// # Parameters
    ///
    /// * `ident_name` should be the sport's name in UpperCamelCase
    /// * `sport_name` should be the human-readable name present in Daktronic's
    ///   documentation.
    /// * Each `field` should follow the format:  
    ///   `(getter_name, field_type, field, "Description", item, length, L/R, "Comment")`
    macro_rules! sport_builder {
        (
            $ident_name:ident,
            $sport_name:literal,
            false,
            $( ($($field:tt)*) ),*
        ) => {
            $crate::sports::macros::__internal_sport_builder_impl!($ident_name,
                $sport_name,
                "# Status: incomplete\n\nThis sport isn't complete (i.e., not all fields have getters), since the maintainer doesn't have infinite time. Help out by transcribing them yourself and [contributing](https://github.com/zabackary/daktronics-allsport-5000-rs).",
                $( ($($field)*) ),*);
        };
        (
            $ident_name:ident,
            $sport_name:literal,
            true,
            $( ($($field:tt)*) ),*
        ) => {
            $crate::sports::macros::__internal_sport_builder_impl!($ident_name,
                $sport_name,
                "## Status: complete\n\nYay! This sport contains a getter for every documented field.",
                $( ($($field)*) ),*);
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __internal_replace_expr {
        ($_t:tt $sub:expr) => {
            $sub
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __internal_count_tts {
        ($($tts:tt)*) => {<[()]>::len(&[$($crate::sports::macros::__internal_replace_expr!($tts ())),*])};
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __internal_sport_builder_impl {
        (
        $ident_name:ident,
        $sport_name:literal,
        $doc_completion_header:literal,
        $( ($($field:tt)*) ),*
    ) => {
            $crate::sports::macros::__internal_paste! {
                #[doc = "A struct providing accessors to the various "
                        "fields of " $sport_name ".\n\nEach of these getters "
                        "returns a different type corresponding to the raw "
                        "data type from the hardware. A list of these fields can "
                        "be found in Daktronic's documentation; see the readme "
                        "of this crate for more details.\n\nTo use " $sport_name
                        "'s accessors, pass this struct as a type parameter to `RTDState`.\n\n# "
                        "Examples\n\n_These examples are auto-generated, so "
                        "they may not work well._\n\n```\n// TODO: examples\n```\n\n"
                        $doc_completion_header
                        ]
                pub struct [<$ident_name Sport>]<DS: $crate::rtd_state::data_source::RTDStateDataSource> {
                    rtd_state: $crate::rtd_state::RTDState<DS>
                }

                impl<DS: $crate::rtd_state::data_source::RTDStateDataSource> $crate::sports::Sport<DS> for [<$ident_name Sport>]<DS> {
                    fn name(&self) -> &'static str {
                        $sport_name
                    }

                    fn rtd_state(&mut self) -> &mut $crate::RTDState<DS> {
                        &mut self.rtd_state
                    }
                }

                impl<DS: $crate::rtd_state::data_source::RTDStateDataSource> [<$ident_name Sport>]<DS> {
                    pub fn new(rtd_state: $crate::RTDState<DS>) -> Self {
                        Self {
                            rtd_state
                        }
                    }

                    $(
                        $crate::sports::macros::__internal_sport_builder_item!($($field)*);
                    )*
                }

                #[cfg(feature = "serde")]
                impl<DS: rtd_state::data_source::RTDStateDataSource> serde::Serialize for [<$ident_name Sport>]<DS> {
                    // since the caller might have 0 arguments, imports might
                    // not be used
                    #[allow(unused)]
                    fn serialize<S: serde::Serializer>(
                        &self,
                        serializer: S
                    ) -> Result<S::Ok, S::Error> {
                        use $crate::sports::macros::__internal_sport_builder_serialize_item as sport_builder_serialize_item;
                        use $crate::sports::macros::__internal_count_tts as count_tts;
                        use serde::ser::SerializeStruct;
                        let mut state = serializer.serialize_struct(
                            stringify!(RTDState<DS, [<$ident_name Sport>]>),
                            // TODO: currently, serializing deprecated fields
                            // is disabled. However, this does not account for
                            // that
                            count_tts!($($( $field )*)*)
                        )?;
                        use $crate::sports::macros::__internal_paste as paste;
                        use $crate::rtd_state::RTDStateFieldError;
                        use serde::ser::Error;
                        $(
                            sport_builder_serialize_item!(state, self, $($field)*);
                        )*
                        state.end()
                    }
                }
            }
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __internal_sport_builder_serialize_item {
        ($state:ident, $self:ident, $field_accessor:ident, $field_type:ty, $field_id:literal, $field_description:literal, $field_item:literal, $field_length:literal, $field_justify:ident, $comment:literal) => {
            paste! {
                $state.serialize_field(stringify!([<$field_accessor:camel>]), &match $self.$field_accessor() {
                    Ok(x) => Some(x),
                    Err(RTDStateFieldError::NoData) => None,
                    Err(e) => return Err(S::Error::custom(e.to_string()))
                })?;
            }
        };
        ($state:ident, $self:ident, $field_accessor:ident, $field_type:ty, $field_id:literal, $field_description:literal, $field_item:literal, $field_length:literal, $field_justify:ident, $comment:literal, deprecate) => {
            {
                // skipping serialization of field because of deprecation:
                stringify!($field_accessor)
            }
        }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __internal_sport_builder_item {
        ($field_accessor:ident, &str, $field_id:literal, $field_description:literal, $field_item:literal, $field_length:literal, $field_justify:ident, $comment:literal, deprecate) => {
            #[doc = concat!(
                                                    "The accessor for the '",
                                                    $field_description,
                                                    "' field. This field returns an `&str`.",
                                                    "\n\n",
                                                    "Field ID #",
                                                    $field_id,
                                                    ". ",
                                                    $comment
                                                )]
            #[deprecated = $comment]
            pub fn $field_accessor(
                &self,
            ) -> Result<&str, $crate::rtd_state::RTDStateFieldError> {
                self.rtd_state.field_str(
                    $field_item,
                    $field_length,
                    $crate::sports::macros::__internal_sport_builder_item!(
                        impl justification,
                        $field_justify
                    ),
                )
            }
        };
        ($field_accessor:ident, &str, $field_id:literal, $field_description:literal, $field_item:literal, $field_length:literal, $field_justify:ident, $comment:literal) => {
            #[doc = concat!(
                                                    "The accessor for the '",
                                                    $field_description,
                                                    "' field. This field returns an `&str`.",
                                                    "\n\n",
                                                    "Field ID #",
                                                    $field_id,
                                                    ". ",
                                                    $comment
                                                )]
            pub fn $field_accessor(
                &self,
            ) -> Result<&str, $crate::rtd_state::RTDStateFieldError> {
                self.rtd_state.field_str(
                    $field_item,
                    $field_length,
                    $crate::sports::macros::__internal_sport_builder_item!(
                        impl justification,
                        $field_justify
                    ),
                )
            }
        };
        ($field_accessor:ident, i32, $field_id:literal, $field_description:literal, $field_item:literal, $field_length:literal, $field_justify:ident, $comment:literal, deprecate) => {
            #[doc = concat!(
                                                    "The accessor for the '",
                                                    $field_description,
                                                    "' field. This field returns an `i32`.",
                                                    "\n\n",
                                                    "Field ID #",
                                                    $field_id,
                                                    ". ",
                                                    $comment
                                                )]
            #[deprecated = $comment]
            pub fn $field_accessor(&self) -> Result<i32, $crate::rtd_state::RTDStateFieldError> {
                self.rtd_state.field_i32(
                    $field_item,
                    $field_length,
                    $crate::sports::macros::__internal_sport_builder_item!(
                        impl justification,
                        $field_justify
                    ),
                )
            }
        };
        ($field_accessor:ident, i32, $field_id:literal, $field_description:literal, $field_item:literal, $field_length:literal, $field_justify:ident, $comment:literal) => {
            #[doc = concat!(
                                                    "The accessor for the '",
                                                    $field_description,
                                                    "' field. This field returns an `i32`.",
                                                    "\n\n",
                                                    "Field ID #",
                                                    $field_id,
                                                    ". ",
                                                    $comment
                                                )]
            pub fn $field_accessor(&self) -> Result<i32, $crate::rtd_state::RTDStateFieldError> {
                self.rtd_state.field_i32(
                    $field_item,
                    $field_length,
                    $crate::sports::macros::__internal_sport_builder_item!(
                        impl justification,
                        $field_justify
                    ),
                )
            }
        };
        ($field_accessor:ident, bool, $field_id:literal, $field_description:literal, $field_item:literal, $field_length:literal, $field_justify:ident, $comment:literal, deprecate) => {
            #[doc = concat!(
                                                    "The accessor for the '",
                                                    $field_description,
                                                    "' field. This field returns a `bool`.",
                                                    "\n\n",
                                                    "Field ID #",
                                                    $field_id,
                                                    ". ",
                                                    $comment
                                                )]
            #[deprecated = $comment]
            pub fn $field_accessor(&self) -> Result<bool, $crate::rtd_state::RTDStateFieldError> {
                self.rtd_state.field_bool($field_item)
            }
        };
        ($field_accessor:ident, bool, $field_id:literal, $field_description:literal, $field_item:literal, $field_length:literal, $field_justify:ident, $comment:literal) => {
            #[doc = concat!(
                                                    "The accessor for the '",
                                                    $field_description,
                                                    "' field. This field returns a `bool`.",
                                                    "\n\n",
                                                    "Field ID #",
                                                    $field_id,
                                                    ". ",
                                                    $comment
                                                )]
            pub fn $field_accessor(&self) -> Result<bool, $crate::rtd_state::RTDStateFieldError> {
                self.rtd_state.field_bool($field_item)
            }
        };
        (impl justification, N) => {
            $crate::rtd_state::RTDFieldJustification::None
        };
        (impl justification, L) => {
            $crate::rtd_state::RTDFieldJustification::Left
        };
        (impl justification, R) => {
            $crate::rtd_state::RTDFieldJustification::Right
        };
    }

    #[doc(hidden)]
    pub use __internal_count_tts;
    #[doc(hidden)]
    pub use __internal_replace_expr;
    #[doc(hidden)]
    pub use __internal_sport_builder_impl;
    #[doc(hidden)]
    pub use __internal_sport_builder_item;
    #[doc(hidden)]
    pub use __internal_sport_builder_serialize_item;
    #[doc(hidden)]
    pub use paste::paste as __internal_paste;
    pub(super) use sport_builder;
}

use macros::sport_builder;
