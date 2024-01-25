macro_rules! destructive_loop {
    ($a:block) => {
        for i in 0..=100_000 {
            dbg!(i);
            if i == 100_000 { panic!("loop never breaked"); };
            $a
        }
    }
}
pub(crate) use destructive_loop;

