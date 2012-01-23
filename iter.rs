type iterable<A> = fn@(fn(A));

fn iterate<A>(coll: iterable<A>, blk: fn(A)) {
    coll(blk);
}

fn enumerate<A>(self: iterable<A>, blk: fn(uint, A)) {
    let i = 0u;
    iterate(self) {|a|
        blk(i, a);
        i += 1u;
    }
}

// Here: we have to use fn@ for predicates and map functions, because
// we will be binding them up into a closure.  Disappointing.  A true
// region type system might be able to do better than this.

fn filter<A>(self: iterable<A>, prd: fn@(A) -> bool, blk: fn(A)) {
    iterate(self) {|a|
        if prd(a) { blk(a) }
    }
}

fn map<A,B>(self: iterable<A>, cnv: fn@(A) -> B, blk: fn(B)) {
    iterate(self) {|a|
        let b = cnv(a);
        blk(b);
    }
}

fn filter_map<A,B>(self: iterable<A>, cnv: fn@(A) -> option<B>, blk: fn(B)) {
    iterate(self) {|a|
        alt cnv(a) {
          some(b) { blk(b) }
          none { /* no action */ }
        }
    }
}

fn foldl<A,B:copy>(self: iterable<A>, b0: B, blk: fn@(B, A) -> B) -> B {
    let b = b0;
    iterate(self) {|a|
        b = blk(b, a);
    }
    ret b;
}

fn to_list<A:copy>(self: iterable<A>) -> [A] {
    foldl(self, [], {|r, a| r + [a]})
}

#[test]
fn test_enumerate() {
    enumerate(bind vec::iter([0u, 1u, 2u], _)) {|i,j|
        assert i == j;
    }
}

#[test]
fn test_to_list() {
    let a = bind vec::iter([0, 1, 2], _);
    let b = bind map(a, {|i| i*2}, _);
    let c = to_list(b);
    assert c == [0, 2, 4];
}