//! **UnionFind\<K\>** is a disjoint-set data structure.

use std::num;

/// **UnionFind\<K\>** is a disjoint-set data structure. It tracks set membership of *n* elements
/// indexed from *0* to *n - 1*. The scalar type is **K** which must be an unsigned integer type.
///
/// http://en.wikipedia.org/wiki/Disjoint-set_data_structure
///
/// Too awesome not to quote:
///
/// “The amortized time per operation is **O(α(n))** where **α(n)** is the
/// inverse of **f(x) = A(x, x)** with **A** being the extremely fast-growing Ackermann function.”
#[derive(Show, Clone)]
pub struct UnionFind<K> where K: num::UnsignedInt
{
    // For element at index *i*, store the index of its parent; the representative itself
    // stores its own index. This forms equivalence classes which are the disjoint sets, each
    // with a unique representative.
    parent: Vec<K>,
    // It is a balancing tree structure,
    // so the ranks are logarithmic in the size of the container -- a byte is more than enough.
    //
    // Rank is separated out both to save space and to save cache in when searching in the parent
    // vector.
    rank: Vec<u8>,
}

#[inline]
fn to_uint<K: num::UnsignedInt>(x: K) -> usize { x.to_uint().unwrap() }

#[inline]
unsafe fn get_unchecked<K>(xs: &[K], index: usize) -> &K
{
    debug_assert!(index < xs.len());
    xs.get_unchecked(index)
}

impl<K> UnionFind<K> where K: num::UnsignedInt
{
    /// Create a new **UnionFind** of **n** disjoint sets.
    pub fn new(n: usize) -> Self
    {
        let mut parent = Vec::with_capacity(n);
        let mut rank = Vec::with_capacity(n);

        for _ in range(0, n) {
            rank.push(0)
        }

        // unroll the first iteration to avoid wraparound in i for K=u8, n=256.
        let mut i: K = num::Int::zero();
        if n > 0 {
            parent.push(i);
        }
        for _ in range(1, n) {
            i = i + num::Int::one();
            parent.push(i);
        }
        UnionFind{parent: parent, rank: rank}
    }

    /// Return the representative for **x**.
    ///
    /// **Panics** if **x** is out of bounds.
    pub fn find(&self, x: K) -> K
    {
        assert!(to_uint(x) < self.parent.len());
        unsafe {
            let mut x = x;
            loop {
                // Use unchecked indexing because we can trust the internal set ids.
                let xparent = *get_unchecked(&*self.parent, to_uint(x));
                if xparent == x {
                    break
                }
                x = xparent;
            }
            x
        }
    }

    /// Return the representative for **x**.
    ///
    /// Write back the found representative, flattening the internal
    /// datastructure in the process and quicken future lookups.
    ///
    /// **Panics** if **x** is out of bounds.
    pub fn find_mut(&mut self, x: K) -> K
    {
        assert!(to_uint(x) < self.parent.len());
        unsafe {
            self.find_mut_recursive(x)
        }
    }

    unsafe fn find_mut_recursive(&mut self, x: K) -> K
    {
        let xparent = *get_unchecked(&*self.parent, to_uint(x));
        if xparent != x {
            let xrep = self.find_mut_recursive(xparent);
            let xparent = self.parent.get_unchecked_mut(to_uint(x));
            *xparent = xrep;
            *xparent
        } else {
            xparent
        }
    }


    /// Unify the two sets containing **x** and **y**.
    ///
    /// Return **false** if the sets were already the same, **true** if they were unified.
    /// 
    /// **Panics** if **x** or **y** is out of bounds.
    pub fn union(&mut self, x: K, y: K) -> bool
    {
        if x == y {
            return false
        }
        let xrep = self.find_mut(x);
        let yrep = self.find_mut(y);

        if xrep == yrep {
            return false
        }

        let xrepu = to_uint(xrep);
        let yrepu = to_uint(yrep);
        let xrank = self.rank[xrepu];
        let yrank = self.rank[yrepu];

        // The rank corresponds roughly to the depth of the treeset, so put the 
        // smaller set below the larger
        if xrank < yrank {
            self.parent[xrepu] = yrep;
        } else if xrank > yrank {
            self.parent[yrepu] = xrep;
        } else {
            // put y below x when equal.
            self.parent[yrepu] = xrep;
            self.rank[xrepu] += 1;
        }
        true
    }
}