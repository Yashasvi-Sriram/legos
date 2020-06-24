#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn function_macro() {
        assert_eq!(function!(), format!("{}::function_macro", module_path!()));
    }
}
