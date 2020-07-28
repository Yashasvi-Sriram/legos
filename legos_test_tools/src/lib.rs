pub mod fitting;
pub mod postprocessing;
pub mod preprocessing;
pub mod proof;

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
            let cp : legos_test_tools::proof::CorrectnessProof = cp();
            println!("{:?}", cp);
        }

        #[test]
        fn time_complexity_proof() {
            let tp : legos_test_tools::proof::ComplexityProof = tp();
            println!("{:?}", tp);
        }

        #[test]
        fn space_complexity_proof() {
            let sp : legos_test_tools::proof::ComplexityProof = sp();
            println!("{:?}", sp);
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
