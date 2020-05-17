use std::ops::{Index, Range};
use std::convert::TryInto;

/// Track a single memory space that has gaps in it's currently populated regions
///
/// For example, consider the case of an executable memory map. Some regions may be unused
#[derive(Debug, Default)]
pub struct SparseMem {
    // really need an interval tree, but there are limited choices in rust right now
    // options:
    //  - https://docs.rs/interval_tree/0.2.0/interval_tree/index.html
    //    - limited query capability, only 1 element can be returned
    //    - has 2 tree variations, unclear which the right one to use 
    //  - https://docs.rs/rangetree/0.1.2/rangetree/
    //    - range representation is strange, uses a 2 element array
    //    - appears to predate standard range representation
    //    - latest update was Feb 2017
    //  - https://docs.rs/segment-tree/2.0.0/segment_tree/
    //    - SegmentPoint: fixed size, 2n elements allcoated
    //    - PointSegment: must start at zero
    //
    // Potential basis for an implimentation:
    //  - linux's augmented rbtree (intrusive)
    //  - some augmented btree
    ranges: Vec<(u64, Vec<u8>)>,
}

impl SparseMem {
    fn ensure_well_formed(&self) {
        // overlaps and adjacent ranges forbidden
        for (i, i_r) in self.ranges.iter().enumerate() {
            let i_e = i_r.0 + i_r.1.len() as u64;
            if i + 1 == self.ranges.len() {
                continue;
            }
            for j in (i + 1)..self.ranges.len() {
                let j_r = &self.ranges[j];
                let j_e = j_r.0 + j_r.1.len() as u64;
                if (i_r.0 < j_e) && (j_r.0 < i_e) {
                    panic!("range {}:({:?}, {}) overlaps with range {}:({:?}, {})", i, i_r, i_e, j, j_r, j_e);
                }

                if i_r.0 == j_e {
                    panic!("range {}:({:?}, {}) adjacent with range {}:({:?}, {})", j, j_r, j_e, i, i_r, i_e);
                }

                if j_r.0 == i_e {
                    panic!("range {}:({:?}, {}) adjacent with range {}:({:?}, {})", i, i_r, i_e, j, j_r, j_e);
                }
            }
        }
    }

    pub fn insert(&mut self, addr: u64, data: &[u8]) -> Result<(), ()> {
        // find overlaping or adjacent entries
        let end = addr.checked_add(data.len() as u64).unwrap();
        let mut appended = None;
        for (i, range) in self.ranges.iter_mut().enumerate() {
            let r_end = range.0 + (range.1.len() as u64);

            // check for overlaps
            if (addr < r_end) && (range.0 < end) {
                // overlap, reject (?)
                return Err(())
            }

            if r_end == addr {
                assert!(appended.is_none());

                // append to this one
                range.1.extend_from_slice(data);
                appended = Some(i);
            }

        }

        // if no prefix found, create ourselves
        let mut appended = match appended {
            None => {
                let a = self.ranges.len();
                self.ranges.push((addr, data.to_owned()));
                a
            },
            Some(a) => a,
        };

        // collect follower into the used segment, if any
        for (i, range) in self.ranges.iter().enumerate() {
            if end == range.0  {
                let follower = self.ranges.remove(i);
                if i < appended {
                    appended -= 1;
                }
                self.ranges[appended].1.extend(follower.1);
                break;
            }
        }

        self.ensure_well_formed();
        Ok(())
    }

    pub fn ranges(&self) -> &[(u64, Vec<u8>)] {
        &self.ranges[..]
    }

    pub fn get(&self, range: Range<u64>) -> Option<&[u8]> {
        for r in self.ranges.iter() {
            if contains_range(&(r.0..(r.0 + r.1.len() as u64)), &range) {
                // this range totally contains the request
                let r_start = (range.start - r.0).try_into().unwrap();
                let r_end = (range.end - r.0).try_into().unwrap();
                return Some(&r.1[r_start..r_end]);
            }
        }

        None
    }
}

fn contains_range(a: &Range<u64>, b: &Range<u64>) -> bool {
    if a.start > b.start {
        return false;
    }

    return a.end >= b.end;
}

impl Index<Range<u64>> for SparseMem {
    type Output = [u8];
    fn index(&self, index: Range<u64>) -> &Self::Output {
        self.get(index).unwrap() 
    }
}
