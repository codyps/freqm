use freqm::sparse_mem::SparseMem;

#[test]
fn insert_extend() {
    let mut s = SparseMem::default();

    assert_eq!(s.insert(0, &[1, 2]), Ok(()));
    assert_eq!(s.insert(10, &[50, 51]), Ok(()));
    assert_eq!(s.insert(1, &[8, 9]), Err(()));
    assert_eq!(s.insert(2, &[3, 4]), Ok(()));
    
    assert_eq!(s.ranges(), &[(0, [1, 2, 3, 4].to_vec()), (10, [50, 51].to_vec())]);
}

#[test]
fn insert_prefix() {
    let mut s = SparseMem::default();

    assert_eq!(s.insert(2, &[3, 4]), Ok(()));
    assert_eq!(s.insert(1, &[9, 10]), Err(()));
    assert_eq!(s.insert(0, &[1, 2]), Ok(()));
    
    assert_eq!(s.ranges(), &[(0, [1, 2, 3, 4].to_vec())]);
}
