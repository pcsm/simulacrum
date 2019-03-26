/// Use this macro to create a `Validator` that works for methods with 2-9 parameters.
#[macro_export]
macro_rules! params {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e: expr, $f: expr, $g: expr, $h: expr, $i: expr) => {
        $crate::Tuple9(
            Box::new($a),
            Box::new($b),
            Box::new($c),
            Box::new($d),
            Box::new($e),
            Box::new($f),
            Box::new($g),
            Box::new($h),
            Box::new($i),
        );
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e: expr, $f: expr, $g: expr, $h: expr) => {
        $crate::Tuple8(
            Box::new($a),
            Box::new($b),
            Box::new($c),
            Box::new($d),
            Box::new($e),
            Box::new($f),
            Box::new($g),
            Box::new($h),
        );
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e: expr, $f: expr, $g: expr) => {
        $crate::Tuple7(
            Box::new($a),
            Box::new($b),
            Box::new($c),
            Box::new($d),
            Box::new($e),
            Box::new($f),
            Box::new($g),
        );
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e: expr, $f: expr) => {
        $crate::Tuple6(
            Box::new($a),
            Box::new($b),
            Box::new($c),
            Box::new($d),
            Box::new($e),
            Box::new($f),
        );
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e: expr) => {
        $crate::Tuple5(
            Box::new($a),
            Box::new($b),
            Box::new($c),
            Box::new($d),
            Box::new($e),
        );
    };
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        $crate::Tuple4(Box::new($a), Box::new($b), Box::new($c), Box::new($d));
    };
    ($a:expr, $b:expr, $c:expr) => {
        $crate::Tuple3(Box::new($a), Box::new($b), Box::new($c));
    };
    ($a:expr, $b:expr) => {
        $crate::Tuple2(Box::new($a), Box::new($b))
    };
}
