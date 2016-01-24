//! http://maniagnosis.crsr.net/2016/01/another-rust-spot-delegation.html

macro_rules! delegate {
    ( $fld:ident : ) => {
    };

    ( $fld:ident : $fcn:ident ( $( $a:ident : $at:ty ),* ) -> $r:ty ) => {
        fn $fcn ( &self, $( $a : $at ),* ) -> $r {
            (self.borrow().$fld).$fcn( $( $a ),* )
        }
    };

    ( $fld:ident : $fcn:ident ( $( $a:ident : $at:ty ),* ) -> $r:ty, $($rest:tt)* ) => {
        fn $fcn ( &self, $( $a : $at ),* ) -> $r {
            (self.borrow().$fld).$fcn( $( $a ),* )
        }
        delegate!($fld : $($rest)*);
    };

    //( $fld:ident : pub $fcn:ident ( $( $a:ident : $at:ty ),* ) -> $r:ty ) => {
    //    pub fn $fcn ( &self, $( $a : $at ),* ) -> $r {
    //        (self.$fld).$fcn( $( $a ),* )
    //    }
    //};

    //( $fld:ident : pub $fcn:ident ( $( $a:ident : $at:ty ),* ) -> $r:ty, $($rest:tt)* ) => {
    //    pub fn $fcn ( &self, $( $a : $at ),* ) -> $r {
    //        (self.$fld).$fcn( $( $a ),* )
    //    }
    //    delegate!($fld : $($rest)*);
    //};

    ( $fld:ident : mut $fcn:ident ( $( $a:ident : $at:ty ),* ) -> $r:ty ) => {
        fn $fcn ( &self, $( $a : $at ),* ) -> $r {
            (self.borrow_mut().$fld).$fcn( $( $a ),* )
        }
    };

    ( $fld:ident : mut $fcn:ident ( $( $a:ident : $at:ty ),* ) -> $r:ty, $($rest:tt)* ) => {
        fn $fcn ( &self, $( $a : $at ),* ) -> $r {
            (self.borrow_mut().$fld).$fcn( $( $a ),* )
        }
        delegate!($fld : $($rest)*);
    };

    //( $fld:ident : pub mut $fcn:ident ( $( $a:ident : $at:ty ),* ) -> $r:ty ) => {
    //    pub fn $fcn ( &mut self, $( $a : $at ),* ) -> $r {
    //        (self.$fld).$fcn( $( $a ),* )
    //    }
    //};

    //( $fld:ident : pub mut $fcn:ident ( $( $a:ident : $at:ty ),* ) -> $r:ty, $($rest:tt)* ) => {
    //    pub fn $fcn ( &mut self, $( $a : $at ),* ) -> $r {
    //        (self.$fld).$fcn( $( $a ),* )
    //    }
    //    delegate!($fld : $($rest)*);
    //};

}
