#[macro_export]
macro_rules! hashmap {
    () => { std::collections::HashMap::new() };

    ( $( $k:expr=>$v:expr ),+ ) => {
        {
            let mut tmp = std::collections::HashMap::new();

            $(
                tmp.insert($k, $v);
            )*

            tmp
        }
    };
}