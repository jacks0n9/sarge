pub mod const_exprs;

#[macro_export]
macro_rules! __parse_arg {
    ( err => $parser:expr, $name:ident ) => {
        let $name = $name.get();
    };

    ( ok => $parser:expr, $name:ident ) => {
        let $name = $name.get().map(|a| a.ok()).flatten();
    };

    ( => $parser:expr, $name:ident ) => {
        let $name = $name
            .get()
            .expect("Tried to unwrap argument that wasn't passed")
            .expect("Tried to unwrap argument that failed to parse");
    };
}

#[macro_export]
macro_rules! __arg_typ {
    ( err , $typ:ty ) => {
        $crate::ArgResult<$typ>
    };

    ( ok , $typ:ty ) => {
        std::option::Option<$typ>
    };

    ( $typ:ty ) => {
        $typ
    };
}

#[macro_export]
macro_rules! __var_tag {
    ( $long:ident ) => {
        $crate::tag::long($crate::__replace!(stringify!($long), '_', '-'))
    };
    ( $short:literal $long:ident ) => {
        $crate::tag::both($short, $crate::__replace!(stringify!($long), '_', '-'))
    };
    ( $long:ident $env:ident ) => {
        $crate::tag::long($crate::__replace!(stringify!($long), '_', '-')).env(stringify!($env))
    };
    ( $short:literal $long:ident $env:ident ) => {
        $crate::tag::both($short, $crate::__replace!(stringify!($long), '_', '-'))
            .env(stringify!($env))
    };
}

#[macro_export]
macro_rules! sarge {
    ( $v:vis $name:ident, $( $( # $spec:ident )? $( $short:literal )? $( @ $env:ident )? $av:vis $long:ident : $typ:ty ),* $(,)? ) => {
        $v struct $name {
            $( $av $long: $crate::__arg_typ!($($spec,)? $typ), )*
        }

        impl $name {
            /// Parse arguments from `std::env::{args,vars}`.
            ///
            /// # Errors
            ///
            /// If any arguments fail to parse their values, this
            /// will forward that error. Otherwise, see
            /// [`ArgParseError`] for a list of all possible errors.
            #[allow(unused)]
            pub fn parse() -> std::result::Result<(Self, std::vec::Vec<std::string::String>), ArgParseError> {
                Self::parse_provided(
                    std::env::args().collect::<std::vec::Vec<_>>().as_slice(),
                    std::env::vars(),
                )
            }

            /// Parse the provided arguments as if they were environment variables.
            ///
            /// If `reset == true`, clears the values of all arguments beforehand.
            /// You probably want to leave this at `false`, unless you're re-using
            /// your parser.
            ///
            /// # Errors
            ///
            /// See [`parse`] for details.
            #[allow(unused)]
            pub fn parse_env<I: std::iter::Iterator<Item = (std::string::String, std::string::String)>>(
                args: I,
            ) -> std::result::Result<Self, $crate::ArgParseError> {
                Ok(Self::parse_provided(
                    &[],
                    args
                )?.0)
            }

            /// Parses the provided arguments as if they were from the CLI.
            ///
            /// If `reset == true`, clears the values of all arguments beforehand.
            /// You probably want to leave this at `false`, unless you're re-using
            /// your parser.
            ///
            /// # Errors
            ///
            /// See [`parse`] for details.
            #[allow(clippy::missing_panics_doc)]
            #[allow(unused)]
            pub fn parse_cli(args: &[std::string::String]) -> std::result::Result<(Self, std::vec::Vec<std::string::String>), $crate::ArgParseError> {
                Self::parse_provided(
                    args,
                    None.into_iter()
                )
            }

            /// Parse from the provided environment variables and CLI arguments.
            ///
            /// # Errors
            ///
            /// See [`parse`] for details.
            #[allow(unused)]
            pub fn parse_provided<I>(
                cli: &[std::string::String],
                env: I,
            ) -> std::result::Result<
                    (Self, std::vec::Vec<std::string::String>), $crate::ArgParseError
                > where
                    I: std::iter::Iterator<Item = (std::string::String, std::string::String)>
            {
                let parser = $crate::ArgumentParser::new();

                $(
                    let $long = parser.add::<$typ>(
                        $crate::__var_tag!($( $short )? $long $( $env )?)
                    );
                )*

                let remainder = parser.parse_provided(cli, env)?;

                $(
                    $crate::__parse_arg!($($spec)? => parser, $long);
                )*

                let me = Self {$(
                    $long,
                )*};

                Ok((me, remainder))
            }
        }
    };
}
