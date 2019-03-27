use std::cmp::{PartialEq, PartialOrd};
use std::iter::Peekable;

/// Returns the first identifier in a repitition
#[macro_export] macro_rules! get_first {
    ($iter:ident, $($iter1:ident,)*) => {$iter}
}

// Generates a KVJoin Struct for the inputed number of arguments
#[macro_export] macro_rules! kvajoiner {
    ($name:ident, $($type:ident, $iter:ident, $loc:tt),*) => {
        #[allow(dead_code)]
        /// Define the Key Value Join Struct for example kvjoiner!(KVJoin2, A, Ai, B, Bi) would generate this struct
        /// struct KVJoin2<K,A,B,Ai,Bi>(KVJoinIterator<K,A,Ai>, KVJoinIterator<K,B,Bi>)
        /// where
        ///     K : PartialEq + PartialOrd,
        ///     Ai : Iterator<Item=(K,A)>,
        ///     Bi : Iterator<Item=(K,B)>;
        pub struct $name<K, $($type,$iter,)*>($($iter,)*)
            where 
                K : PartialEq + PartialOrd,
                $($iter : Iterator<Item=(K,$type)>,)*;
        
        /// Defines the from trait to convert a tuple of iterators into a specific KVAJoin
        /// for example kvjoiner!(KVJoin2, A, Ai, B, Bi) would generate this implementation
        /// impl <K, A, Ai, B, Bi> From<(Ai, Bi)> for KVJoin2<K, A, Ai, B, Bi> where
        /// K: PartialEq + PartialOrd, 
        /// Ai: Iterator<Item = (K, A)> + Into<KVJoinIterator<K, A, Ai>>,
        /// Bi: Iterator<Item = (K, B)> + Into<KVJoinIterator<K, B, Bi>> {
        ///     fn from(iters: (Ai, Bi)) -> Self {
        ///         Self(KVJoinIterator::from(iters.0), KVJoinIterator::from(iters.1))
        ///     }
        /// }
        impl<K, $($type,$iter,)*> From<($($iter,)*)> for $name<K, $($type,$iter,)*>
            where
                K : PartialEq + PartialOrd,
                $($iter : Iterator<Item=(K,$type)>,)* {
            fn from(iters : ($($iter,)*)) -> Self {
                Self( $(iters.$loc),* )
            }
        }
        
        /// Defines the from trait to convert a tuple of iterators into a Generic KVJoin
        impl<K, $($type,$iter,)*> From<($($iter,)*)> for KVAJoin<K, ($($type,)*), $name<K, $($type,$iter,)*>>
            where
                K : PartialEq + PartialOrd + Clone + Copy,
                $($iter : Iterator<Item=(K,$type)>,)* {
            fn from(iters : ($($iter,)*)) -> Self {
                Self( $name::from(iters) )
            }
        }

        /// Implements Iterator for the KVJoin Iterator
        impl<K, $($type,$iter,)*> Iterator for $name<K, $($type,$iter,)*>
            where
                K : PartialEq + PartialOrd + Clone + Copy,
                $($iter : Iterator<Item=(K,$type)>,)* {
            type Item=(K, ($($type,)*));

            fn next(&mut self) -> Option<(K, ($($type,)*))> {
                // First step is to find all the first keys and values
                $(
                    #[allow(non_snake_case)]
                    let (mut $iter, mut $type) = match self.$loc.next() {
                        Some((k,v)) => (k,v),
                        None => return None,
                    };
                    
                )*
                // Second step is to find the largest key value
                let mut k = get_first!( $($iter,)* );
                // Third step is to iterate towards the highest value
                'outer: loop {
                    $(
                        loop {
                            // If iter is < then k then iterate
                            if $iter < k {
                                if let Some((kt, vt)) = self.$loc.next() {
                                    $iter = kt;
                                    $type = vt;
                                } else {
                                    return None;
                                }
                            // If iter is > then k then there is no k key in the iterator,
                            // therefore there is no need to join on this value of k so start
                            // over with the new higher value of k for all collections
                            } else if $iter > k {
                                k = $iter;
                                continue 'outer;
                            // If this is equal to k then the current value is already saved
                            } else {
                                break;
                            }
                        }
                    )*
                    // If this is reached then all iters generated a value with key k
                    return Some((k, ($($type,)*)))
                }
            }
        }
    }
}

pub struct KVAJoin<K : PartialOrd + PartialEq, V, I : Iterator<Item=(K, V)>>(I);

impl<K : PartialOrd + PartialEq, V, I : Iterator<Item=(K, V)>> Iterator  for KVAJoin<K, V, I>{
    type Item = (K, V);
    fn next(&mut self) -> Option<(K, V)> {
        self.0.next()
    }
}

// Generates a KVJoin Struct for the inputed number of arguments
#[macro_export] macro_rules! kvojoiner {
    ($name:ident, $($type:ident, $iter:ident, $loc:tt),*) => {
        #[allow(dead_code)]
        /// Define the Key Value Join Struct for example kvjoiner!(KVJoin2, A, Ai, B, Bi) would generate this struct
        /// struct KVJoin2<K,A,B,Ai,Bi>(KVJoinIterator<K,A,Ai>, KVJoinIterator<K,B,Bi>)
        /// where
        ///     K : PartialEq + PartialOrd,
        ///     Ai : Iterator<Item=(K,A)>,
        ///     Bi : Iterator<Item=(K,B)>;
        pub struct $name<K, $($type,$iter,)*>($(Peekable<$iter>,)*)
            where 
                K : PartialEq + PartialOrd,
                $($iter : Iterator<Item=(K,$type)>,)*;
        
        /// Defines the from trait to convert a tuple of iterators into a specific KVAJoin
        /// for example kvjoiner!(KVJoin2, A, Ai, B, Bi) would generate this implementation
        /// impl <K, A, Ai, B, Bi> From<(Ai, Bi)> for KVJoin2<K, A, Ai, B, Bi> where
        /// K: PartialEq + PartialOrd, 
        /// Ai: Iterator<Item = (K, A)> + Into<KVJoinIterator<K, A, Ai>>,
        /// Bi: Iterator<Item = (K, B)> + Into<KVJoinIterator<K, B, Bi>> {
        ///     fn from(iters: (Ai, Bi)) -> Self {
        ///         Self(KVJoinIterator::from(iters.0), KVJoinIterator::from(iters.1))
        ///     }
        /// }
        impl<K, $($type,$iter,)*> From<($($iter,)*)> for $name<K, $($type,$iter,)*>
            where
                K : PartialEq + PartialOrd,
                $($iter : Iterator<Item=(K,$type)>,)* {
            fn from(iters : ($($iter,)*)) -> Self {
                Self( $(iters.$loc.peekable()),* )
            }
        }
        
        /// Defines the from trait to convert a tuple of iterators into a Generic KVJoin
        impl<K, $($type,$iter,)*> From<($($iter,)*)> for KVOJoin<K, ($(Option<$type>,)*), $name<K, $($type,$iter,)*>>
            where
                K : PartialEq + PartialOrd + Clone + Copy,
                $($iter : Iterator<Item=(K,$type)>,)* {
            fn from(iters : ($($iter,)*)) -> Self {
                Self( $name::from(iters) )
            }
        }

        /// Implements Iterator for the KVJoin Iterator
        impl<K, $($type,$iter,)*> Iterator for $name<K, $($type,$iter,)*>
            where
                K : PartialEq + PartialOrd + Clone + Copy,
                $($iter : Iterator<Item=(K,$type)>,)* {
            type Item=(K, ($(Option<$type>,)*));

            fn next(&mut self) -> Option<(K, ($(Option<$type>,)*))> {
               // Step 1: Find minimum K
                $(
                    // we are using the $iter identifier as a variable, however in this case
                    // it stores the value of the key, I know the naming sucks but i couldnt
                    // find a way around this
                    #[allow(non_snake_case)]
                    let ($iter, _) = match self.$loc.peek() {
                        Some((k,_v)) => (k,_v),
                        None => return None,
                    };
                )*
                // inititialize the minimum value of the key to the first iterators
                // peeked key value
                let mut k = get_first!( $($iter,)* );
                // iterate through the iterators to find the smalles key
                $(
                    if $iter < k {
                        k = $iter;
                    }
                )*
                // in order to get rid of the first mutable borrow to call
                // next we use the $type identifier to store a boolean of
                // wether or not the key is equal to the min key
                $(
                    #[allow(non_snake_case)]
                    let $type = $iter == k;
                )*
                Some((*k,($(
                    if $type {
                        match self.$loc.next() {
                            Some((_k, v)) => Some(v),
                            None => return None,
                        }
                    } else {
                        None
                    }
                ,)*)))
               // Step 2: construct tuple of options if type is min_k
               // step 3: return tuple of options
            }
        }
    }
}

pub struct KVOJoin<K : PartialOrd + PartialEq, V, I : Iterator<Item=(K, V)>>(I);

impl<K : PartialOrd + PartialEq, V, I : Iterator<Item=(K, V)>> Iterator  for KVOJoin<K, V, I>{
    type Item = (K, V);
    fn next(&mut self) -> Option<(K, V)> {
        self.0.next()
    }
}
//kvojoiner!(KVOJoin2,V0,V0i,0,V1,V1i,1);

/// Creates a KVJoin struct with template arugments
kvajoiner!(KVAJoin2,V0,V0i,0,V1,V1i,1);
kvajoiner!(KVAJoin3,V0,V0i,0,V1,V1i,1,V2,V2i,2);
kvajoiner!(KVAJoin4,V0,V0i,0,V1,V1i,1,V2,V2i,2,V3,V3i,3);
kvajoiner!(KVAJoin5,V0,V0i,0,V1,V1i,1,V2,V2i,2,V3,V3i,3,V4,V4i,4);
kvajoiner!(KVAJoin6,V0,V0i,0,V1,V1i,1,V2,V2i,2,V3,V3i,3,V4,V4i,4,V5,V5i,5);
kvajoiner!(KVAJoin7,V0,V0i,0,V1,V1i,1,V2,V2i,2,V3,V3i,3,V4,V4i,4,V5,V5i,5,V6,V6i,6);
kvajoiner!(KVAJoin8,V0,V0i,0,V1,V1i,1,V2,V2i,2,V3,V3i,3,V4,V4i,4,V5,V5i,5,V6,V6i,6,V7,V7i,7);
kvajoiner!(KVAJoin9,V0,V0i,0,V1,V1i,1,V2,V2i,2,V3,V3i,3,V4,V4i,4,V5,V5i,5,V6,V6i,6,V7,V7i,7,V8,V8i,8);
kvajoiner!(KVAJoin10,V0,V0i,0,V1,V1i,1,V2,V2i,2,V3,V3i,3,V4,V4i,4,V5,V5i,5,V6,V6i,6,V7,V7i,7,V8,V8i,8,V9,V9i,9);

/// Join macro to find the correct KVJoin struct for the join. The Join macro only supports a join of up to 10 iterators of type (K, V), however
/// the join macro also returns an iterator of type (K, V). If more than 10 items are to be joined, nest the join macros, 
/// join!(join!(A,B,C,D,E,F,G), join!(H,I,J,K,L,M))
#[macro_export] macro_rules! kvand_join {
    ($V0:expr,$V1:expr) => {kv_join::KVAJoin::from(($V0,$V1))};
    ($V0:expr,$V1:expr,$V2:expr) => {kv_join::KVAJoin::from(($V0,$V1,$V2))};
    ($V0:expr,$V1:expr,$V2:expr,$V3:expr) => {kv_join::KVAJoin::from(($V0,$V1,$V2,$V3))};
    ($V0:expr,$V1:expr,$V2:expr,$V3:expr,$V4:expr) => {kv_join::KVAJoin::from(($V0,$V1,$V2,$V3,$V4))};
    ($V0:expr,$V1:expr,$V2:expr,$V3:expr,$V4:expr,$V5:expr) => {kv_join::KVAJoin::from(($V0,$V1,$V2,$V3,$V4,$V5))};
    ($V0:expr,$V1:expr,$V2:expr,$V3:expr,$V4:expr,$V5:expr,$V6:expr) => {kv_join::KVAJoin::from(($V0,$V1,$V2,$V3,$V4,$V5,$V6))};
    ($V0:expr,$V1:expr,$V2:expr,$V3:expr,$V4:expr,$V5:expr,$V6:expr,$V7:expr) => {kv_join::KVAJoin::from(($V0,$V1,$V2,$V3,$V4,$V5,$V6,$V7))};
    ($V0:expr,$V1:expr,$V2:expr,$V3:expr,$V4:expr,$V5:expr,$V6:expr,$V7:expr,$V8:expr) => {kv_join::KVAJoin::from(($V0,$V1,$V2,$V3,$V4,$V5,$V6,$V7,$V8))};
    ($V0:expr,$V1:expr,$V2:expr,$V3:expr,$V4:expr,$V5:expr,$V6:expr,$V7:expr,$V8:expr,$V9:expr) => {kv_join::KVAJoin::from(($V0,$V1,$V2,$V3,$V4,$V5,$V6,$V7,$V8,$V9))};
}


/// Creates a KVJoin struct with template arugments
kvojoiner!(KVOJoin2,V0,V0i,0,V1,V1i,1);
kvojoiner!(KVOJoin3,V0,V0i,0,V1,V1i,1,V2,V2i,2);
kvojoiner!(KVOJoin4,V0,V0i,0,V1,V1i,1,V2,V2i,2,V3,V3i,3);
kvojoiner!(KVOJoin5,V0,V0i,0,V1,V1i,1,V2,V2i,2,V3,V3i,3,V4,V4i,4);
kvojoiner!(KVOJoin6,V0,V0i,0,V1,V1i,1,V2,V2i,2,V3,V3i,3,V4,V4i,4,V5,V5i,5);
kvojoiner!(KVOJoin7,V0,V0i,0,V1,V1i,1,V2,V2i,2,V3,V3i,3,V4,V4i,4,V5,V5i,5,V6,V6i,6);
kvojoiner!(KVOJoin8,V0,V0i,0,V1,V1i,1,V2,V2i,2,V3,V3i,3,V4,V4i,4,V5,V5i,5,V6,V6i,6,V7,V7i,7);
kvojoiner!(KVOJoin9,V0,V0i,0,V1,V1i,1,V2,V2i,2,V3,V3i,3,V4,V4i,4,V5,V5i,5,V6,V6i,6,V7,V7i,7,V8,V8i,8);
kvojoiner!(KVOJoin10,V0,V0i,0,V1,V1i,1,V2,V2i,2,V3,V3i,3,V4,V4i,4,V5,V5i,5,V6,V6i,6,V7,V7i,7,V8,V8i,8,V9,V9i,9);

/// Join macro to find the correct KVJoin struct for the join. The Join macro only supports a join of up to 10 iterators of type (K, V), however
/// the join macro also returns an iterator of type (K, V). If more than 10 items are to be joined, nest the join macros, 
/// join!(join!(A,B,C,D,E,F,G), join!(H,I,J,K,L,M))
#[macro_export] macro_rules! kvor_join {
    ($V0:expr,$V1:expr) => {kv_join::KVOJoin::from(($V0,$V1))};
    ($V0:expr,$V1:expr,$V2:expr) => {kv_join::KVOJoin::from(($V0,$V1,$V2))};
    ($V0:expr,$V1:expr,$V2:expr,$V3:expr) => {kv_join::KVOJoin::from(($V0,$V1,$V2,$V3))};
    ($V0:expr,$V1:expr,$V2:expr,$V3:expr,$V4:expr) => {kv_join::KVOJoin::from(($V0,$V1,$V2,$V3,$V4))};
    ($V0:expr,$V1:expr,$V2:expr,$V3:expr,$V4:expr,$V5:expr) => {kv_join::KVOJoin::from(($V0,$V1,$V2,$V3,$V4,$V5))};
    ($V0:expr,$V1:expr,$V2:expr,$V3:expr,$V4:expr,$V5:expr,$V6:expr) => {kv_join::KVOJoin::from(($V0,$V1,$V2,$V3,$V4,$V5,$V6))};
    ($V0:expr,$V1:expr,$V2:expr,$V3:expr,$V4:expr,$V5:expr,$V6:expr,$V7:expr) => {kv_join::KVOJoin::from(($V0,$V1,$V2,$V3,$V4,$V5,$V6,$V7))};
    ($V0:expr,$V1:expr,$V2:expr,$V3:expr,$V4:expr,$V5:expr,$V6:expr,$V7:expr,$V8:expr) => {kv_join::KVOJoin::from(($V0,$V1,$V2,$V3,$V4,$V5,$V6,$V7,$V8))};
    ($V0:expr,$V1:expr,$V2:expr,$V3:expr,$V4:expr,$V5:expr,$V6:expr,$V7:expr,$V8:expr,$V9:expr) => {kv_join::KVOJoin::from(($V0,$V1,$V2,$V3,$V4,$V5,$V6,$V7,$V8,$V9))};
}
