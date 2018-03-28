/// Use this macro to create a `Validator` that works for methods with 2-9 parameters.
#[macro_export]
macro_rules! params {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e: expr, $f: expr, $g: expr, $h: expr, $i: expr) => {
        $crate::validators::Tuple9(Box::new($a), Box::new($b), Box::new($c), Box::new($d), Box::new($e), Box::new($f), Box::new($g), Box::new($h), Box::new($i));
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e: expr, $f: expr, $g: expr, $h: expr) => {
        $crate::validators::Tuple8(Box::new($a), Box::new($b), Box::new($c), Box::new($d), Box::new($e), Box::new($f), Box::new($g), Box::new($h));
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e: expr, $f: expr, $g: expr) => {
        $crate::validators::Tuple7(Box::new($a), Box::new($b), Box::new($c), Box::new($d), Box::new($e), Box::new($f), Box::new($g));
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e: expr, $f: expr) => {
        $crate::validators::Tuple6(Box::new($a), Box::new($b), Box::new($c), Box::new($d), Box::new($e), Box::new($f));
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e: expr) => {
        $crate::validators::Tuple5(Box::new($a), Box::new($b), Box::new($c), Box::new($d), Box::new($e));
    };
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        $crate::validators::Tuple4(Box::new($a), Box::new($b), Box::new($c), Box::new($d));
    };
    ($a:expr, $b:expr, $c:expr) => {
        $crate::validators::Tuple3(Box::new($a), Box::new($b), Box::new($c));
    };
    ($a:expr, $b:expr) => {
        $crate::validators::Tuple2(Box::new($a), Box::new($b))
    };
}
