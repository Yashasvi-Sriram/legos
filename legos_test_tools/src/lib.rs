pub mod fitting;
pub mod postprocessing;
pub mod preprocessing;

#[macro_export]
macro_rules! function_path {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}

#[macro_export]
macro_rules! test_suite {
    ($a:ident, $($b:ident),*) => {
        #[test]
        fn correctness_proof() {
            cp();
        }

        #[test]
        fn time_complexity_proof() {
            tp();
        }

        #[test]
        fn space_complexity_proof() {
            sp();
        }

        #[test]
        fn correctness_tests() {
            // Atleast one correctness test should exist
            $a();
            // More are fine
            $( $b(); )*
        }

        #[test]
        fn time_complexity_test() {
            tt();
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn function_macro() {
        assert_eq!(
            function_path!(),
            format!("{}::function_macro", module_path!())
        );
    }
}
