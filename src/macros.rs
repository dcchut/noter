#[macro_export]
macro_rules! variant {
    ( $e: expr, $n: expr, $s: expr) => {
        $crate::configs::NoteVariant::new($e, $n, $s)
    };
}

#[macro_export]
macro_rules! configuration {
    ( $d: expr, $f: expr, $t: expr, $i: expr $(,$e: expr)*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($e);
            )*
            $crate::configs::Configuration::new($d, $f, $t, $i, temp_vec)
        }
    }
}
