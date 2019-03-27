#[allow(unused_imports)]
#[macro_use] extern crate kv_join;

#[test]
fn test_kvand_join() {
    use std::collections::BTreeMap;

    let mut ints = BTreeMap::new();
    ints.insert(0,0);
    ints.insert(2,2);
    ints.insert(3,3);

    let mut floats = BTreeMap::new();
    floats.insert(0,0.0);
    floats.insert(1,0.1);
    floats.insert(3,0.3);

    let mut iter = kvand_join!(ints.iter(), floats.iter());
    let (k , (v1, v2)) = iter.next().unwrap();
    assert!(*k == 0, "Key of first iteration is incorrect");
    assert!(*v1 == 0, "First value of first iteration is incorrect");
    assert!(*v2 == 0.0, "Second value of first iteration is incorrect");
    let (k , (v1, v2)) = iter.next().unwrap();
    assert!(*k == 3, "Key of second iteration is incorrect");
    assert!(*v1 == 3, "First value of second iteration is incorrect");
    assert!(*v2 == 0.3, "Second value of second iteration is incorrect");
    assert!(iter.next().is_none());
}

#[test]
fn test_mut_kvand_join() {
    use std::collections::BTreeMap;

    let mut ints = BTreeMap::new();
    ints.insert(0,0);
    ints.insert(1,1);
    ints.insert(2,2);
    ints.insert(3,3);

    let mut floats = BTreeMap::new();
    floats.insert(0,0.0);
    floats.insert(1,0.1);
    floats.insert(2,0.2);
    floats.insert(3,0.3);

    for (_, (v1, v2)) in kvand_join!(ints.iter_mut(), floats.iter_mut()){
        *v1 += 1;
        *v2 += 1.0;
    }

    let mut i = 0;
    for (k, (v1, v2)) in kvand_join!(ints.iter(), floats.iter()){
        assert!(*k == i,  format!("Key of iteration is incorrect, expected {}, got {}", i, *k));
        assert!(*v1 == k + 1,  format!("First value of iteration is incorrect, expected {}, got {}", k + 1, *v1));
        assert!(*v2 == 1.0 + *k as f64 * 0.1, format!("Second value of iteration is incorrect, expected {}, got {}", 1.0 + *k as f64 * 0.1, *v2));
        i = i + 1;
    }
}

#[test]
fn test_mult_kvand_join() {
    use std::collections::BTreeMap;

    let mut ints = BTreeMap::new();
    ints.insert(0,0);
    ints.insert(1,1);
    ints.insert(2,2);
    ints.insert(3,3);

    let mut floats = BTreeMap::new();
    floats.insert(0,0.0);
    floats.insert(1,0.1);
    floats.insert(2,0.2);
    floats.insert(3,0.3);

    let mut bools = BTreeMap::new();
    bools.insert(0,true);
    bools.insert(1,false);
    bools.insert(2,true);
    bools.insert(3,false);

    for (_, (v1, v2)) in kvand_join!(ints.iter_mut(), floats.iter_mut()){
        *v1 += 1;
        *v2 += 1.0;
    }

    let mut i = 0;
    for (k, (v1, v2, v3)) in kvand_join!(ints.iter(), floats.iter(), bools.iter()){
        assert!(*k == i,  format!("Key of iteration is incorrect, expected {}, got {}", i, *k));
        assert!(*v1 == k + 1,  format!("First value of iteration is incorrect, expected {}, got {}", k + 1, *v1));
        assert!(*v2 == 1.0 + *k as f64 * 0.1, format!("Second value of iteration is incorrect, expected {}, got {}", 1.0 + *k as f64 * 0.1, *v2));
        assert!(*v3 == (k % 2 == 0), format!("Third value of iteration is incorrect, expected {}, got {}", k % 2 == 0, *v2));
        i = i + 1;
    }
}

#[test]
fn test_kvor_join() {
    use std::collections::BTreeMap;

    let mut ints = BTreeMap::new();
    ints.insert(0,0);
    ints.insert(2,2);
    ints.insert(3,3);

    let mut floats = BTreeMap::new();
    floats.insert(0,0.0);
    floats.insert(1,0.1);
    floats.insert(3,0.3);

    let mut iter = kvor_join!(ints.iter(), floats.iter());
    let (k , (v1, v2)) = iter.next().unwrap();
    assert!(*k == 0, "Key of first iteration is incorrect");
    assert!(*v1.expect("First value of first iteration is None") == 0, "First value of first iteration is incorrect");
    assert!(*v2.expect("Second value of first iteration is None") == 0.0, "Second value of first iteration is incorrect");
    let (k , (v1, v2)) = iter.next().unwrap();
    assert!(*k == 1, "Key of second iteration is incorrect");
    assert!(v1.is_none(), "First value of second iteration is incorrect");
    assert!(*v2.expect("Second value of second iteration is None") == 0.1, "Second value of second iteration is incorrect");
    let (k , (v1, v2)) = iter.next().unwrap();
    assert!(*k == 2, "Key of third iteration is incorrect");
    assert!(*v1.expect("First value of third iteration is None") == 2, "First value of third iteration is incorrect");
    assert!(v2.is_none(), "Second value of third iteration is incorrect");
    let (k , (v1, v2)) = iter.next().unwrap();
    assert!(*k == 3, "Key of fourth iteration is incorrect");
    assert!(*v1.expect("First value of Fourth iteration is None") == 3, "First value of Fourth iteration is incorrect");
    assert!(*v2.expect("Second value of Fourth iteration is None") == 0.3, "Second value of Fourth iteration is incorrect");
    assert!(iter.next().is_none());
}