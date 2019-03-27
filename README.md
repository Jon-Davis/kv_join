# kv_join
kv_join is a rust crate for performing linear time joins on sorted iterators that produce an item of type (Key, Value)

## Performance
The performance hasn't been fully measured. The joins are performed in linear time and there is no dynamic typing or heap allocation.

## kvand_join
The first join is the AND join and is called with the following macro:
```rust 
kvand_join!()
```
What this macro does is produce an iterator that yeilds whenever all of the input iterators yeild the same key. for example the following code
will print 2 lines ``(0, (0, 0.0))`` and ``(3, (3, 0.3))``
```rust
let mut ints = BTreeMap::new();
ints.insert(0,0);
ints.insert(2,2);
ints.insert(3,3);

let mut floats = BTreeMap::new();
floats.insert(0,0.0);
floats.insert(1,0.1);
floats.insert(3,0.3);

for (k, (i,f)) in kvand_join!(ints.iter(), floats.iter()){
  println!("({}, ({}, {}))",k,i,f);
}
```
## kvor_join
The other join is the OR join and is called with the following macro:
```rust 
kvor_join!()
```
What this macro does is produce an iterator that yeilds whenever any of the input iterators yeild a key. for example the following iterator
will yeild ``(0, (Some(0), Some(0.0))`` ``(1, (None, Some(0.1))`` ``(2, (Some(2), None))`` and ``(3, (Some(3), Some(0.3)))``
```rust
let mut ints = BTreeMap::new();
ints.insert(0,0);
ints.insert(2,2);
ints.insert(3,3);

let mut floats = BTreeMap::new();
floats.insert(0,0.0);
floats.insert(1,0.1);
floats.insert(3,0.3);

let mut iter = kvor_join!(ints.iter(), floats.iter());
```
## Mutability
Mutability of an item is based on the mutability of it's iterator, mutable iterators produce mutable refrences, immutable iterators produce
immutable refrences.
```rust
kvand_join!(a.iter(), b.iter_mut()) // you can mutate b, but not a
```
## Nesting
Both the ``kvand_join!()`` and the ``kvor_join!()`` produce sorted iterators of type (K, V) where V is a tuple. This means that the
two can be nested to produce more complicated logic:
```rust
for (k, v) in kvor_join!(kvand_join!(a,b,c), d){
   match v {
      ((av, bv, cv), Some(dv)) => (), //a and b and c and d
      ((av, bv, cv), None) => (), // a and b and c and not d
   }
}
```
Additionally there is a limit to the number of iterators that can be joined. For compile time reasons this limit is currently set to 10, however you can get around
this by nesting the joins.
```rust
kvand_join!(
  kvand_join!(a,b,c,d,e,f,g,h,i,j),
  kvand_join!(l,m,n,o,p,q,r,s,t,u),
)
```
