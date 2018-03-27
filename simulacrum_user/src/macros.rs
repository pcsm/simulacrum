/// Use this macro to create a `Validator` that works for methods with 2-9 parameters.
#[macro_export]
macro_rules! params {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e: expr, $f: expr, $g: expr, $h: expr, $i: expr) => {
        Tuple9(Box::new($a), Box::new($b), Box::new($c), Box::new($d), Box::new($e), Box::new($f), Box::new($g), Box::new($h), Box::new($i));
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e: expr, $f: expr, $g: expr, $h: expr) => {
        Tuple8(Box::new($a), Box::new($b), Box::new($c), Box::new($d), Box::new($e), Box::new($f), Box::new($g), Box::new($h));
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e: expr, $f: expr, $g: expr) => {
        Tuple7(Box::new($a), Box::new($b), Box::new($c), Box::new($d), Box::new($e), Box::new($f), Box::new($g));
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e: expr, $f: expr) => {
        Tuple6(Box::new($a), Box::new($b), Box::new($c), Box::new($d), Box::new($e), Box::new($f));
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e: expr) => {
        Tuple5(Box::new($a), Box::new($b), Box::new($c), Box::new($d), Box::new($e));
    };
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
