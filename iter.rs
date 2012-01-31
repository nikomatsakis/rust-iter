iface iterable<A> {
    fn iter(blk: fn(A));
}

impl<A> of iterable<A> for fn@(fn(A)) {
    fn iter(blk: fn(A)) {
        self(blk);
    }
}

impl<A> of iterable<A> for [A] {
    fn iter(blk: fn(A)) {
        vec::iter(self, blk)
    }
}

impl<A> of iterable<A> for option<A> {
    fn iter(blk: fn(A)) {
        option::may(self, blk)
    }
}

fn enumerate<A,IA:iterable<A>>(self: IA, blk: fn(uint, A)) {
    let i = 0u;
    self.iter {|a|
        blk(i, a);
        i += 1u;
    }
}

// Here: we have to use fn@ for predicates and map functions, because
// we will be binding them up into a closure.  Disappointing.  A true
// region type system might be able to do better than this.

fn filter<A,IA:iterable<A>>(self: IA, prd: fn@(A) -> bool, blk: fn(A)) {
    self.iter {|a|
        if prd(a) { blk(a) }
    }
}

fn map<A,B,IA:iterable<A>>(self: IA, cnv: fn@(A) -> B, blk: fn(B)) {
    self.iter {|a|
        let b = cnv(a);
        blk(b);
    }
}

fn flat_map<A,B,IA:iterable<A>,IB:iterable<B>>(
    self: IA, cnv: fn@(A) -> IB, blk: fn(B)) {
    self.iter {|a|
        cnv(a).iter(blk)
    }
}

fn foldl<A,B:copy,IA:iterable<A>>(self: IA, b0: B, blk: fn(B, A) -> B) -> B {
    let b = b0;
    self.iter {|a|
        b = blk(b, a);
    }
    ret b;
}

fn to_list<A:copy,IA:iterable<A>>(self: IA) -> [A] {
    foldl::<A,[A],IA>(self, [], {|r, a| r + [a]})
}

fn repeat(times: uint, blk: fn(uint)) {
    let i = 0u;
    while i < times {
        blk(i);
        i += 1u;
    }
}


#[test]
fn test_enumerate() {
    enumerate(bind vec::iter([0u, 1u, 2u], _)) {|i,j|
        assert i == j;
    }
}

#[test]
fn test_map_and_to_list() {
    let a = bind vec::iter([0, 1, 2], _);
    let b = bind map(a, {|i| i*2}, _);
    let c = to_list(b);
    assert c == [0, 2, 4];
}

#[test]
fn test_flat_map_with_option() {
    fn if_even(&&i: int) -> option<int> {
        if (i % 2) == 0 { some(i) }
        else { none }
    }

    let a = bind vec::iter([0, 1, 2], _);
    let b = bind flat_map(a, if_even, _);
    let c = to_list(b);
    assert c == [0, 2];
}

#[test]
fn test_flat_map_with_list() {
    fn repeat(&&i: int) -> [int] {
        let r = [];
        int::range(0, i) {|_j| r += [i]; }
        r
    }

    let a = bind vec::iter([0, 1, 2, 3], _);
    let b = bind flat_map(a, repeat, _);
    let c = to_list(b);
    #debug["c = %?", c];
    assert c == [1, 2, 2, 3, 3, 3];
}

#[test]
fn test_repeat() {
    let c = [];
    repeat(5u) {|i|
        c += [(i * 2u)];
    };
    #debug["c = %?", c];
    assert c == [0u, 2u, 4u, 6u, 8u];
}


