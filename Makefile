all:
	rustc iter.rc

test:
	rustc --test iter.rc
	./iter

clean:
	rm -rf libiter-*
	rm -rf iter iter.dSYM