use std::time::Instant;

macro_rules! new_test {
    ($a:expr, $b:expr)=>{
        Test {
            name: $a,
            total_ns: 0,
            count: 0,
            avg: 0,
            func: $b
        }
    }
}

pub struct Test<'a> {
    pub name: &'a str,
    pub total_ns: u128,
    pub count: u128,
    pub avg: u128,
    func: fn()
}

impl Test<'static> {
    pub fn exec<'a>(&mut self) {
        let start = Instant::now();
        &(self.func)();
        let end = start.elapsed();
        self.total_ns += end.as_nanos();
        self.count += 1;
        self.avg = self.total_ns / self.count;
    }
}

pub static mut TESTS: [Test; 2] = [
    new_test!(
        "Multi",
        || {
            10.0 * 0.5;
            ()
        }
    ),
    new_test!(
        "Div",
        || {
            10 / 2;
            ()
        }
    )
];
