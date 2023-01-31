#[macro_export]
macro_rules! hashmap {
    () => { HashMap::new() };

    ( $( $k:expr=>$v:expr ),+ ) => {
        {
            let mut tmp = HashMap::new();

            $(
                tmp.insert($k, $v);
            )*

            tmp
        }
    };
}