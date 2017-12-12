/// Use this macro to create a `Validator` that works for methods with 2, 3, or 
/// 4 parameters.
#[macro_export]
macro_rules! params {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        Tuple4(Box::new($a), Box::new($b), Box::new($c), Box::new($d));
    };
    ($a:expr, $b:expr, $c:expr) => {
        Tuple3(Box::new($a), Box::new($b), Box::new($c));
    };
    ($a:expr, $b:expr) => {
        Tuple2(Box::new($a), Box::new($b))
    };
}