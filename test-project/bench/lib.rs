#![feature(test)]

#[allow(soft_unstable)]
#[cfg(test)]
mod test {
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_something(b: &mut Bencher) {
        b.iter(|| {
            for i in 1..100 {
                let x = i * i;
                assert_eq!(x, i * i);
            }
        });
    }
}
