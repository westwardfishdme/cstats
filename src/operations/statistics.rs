use std::fmt::{self, Display, Pointer};

#[derive(Debug, Copy, Clone)]
pub struct StatData {
    // simple stats values
    pub sum: f64,
    pub count: usize,
    pub avg: f64,

    // range values
    pub min: f64,
    pub max: f64,

    // standard deviation values
    pub sigma: f64,              // population based
    pub standard_deviation: f64, // sample values
}

impl StatData {
    pub fn new(values: Box<[f64]>) -> StatData {
        let x = StatData {
            sum: values.sum(),
            count: values.len(),
            avg: values.average(),
            min: values.min(),
            max: values.max(),

            sigma: 0_f64,
            standard_deviation: 0_f64,
        };
        let mut y = x;
        y.sigma = y.sigma(&values);
        y.standard_deviation = y.standard_deviation(&values);

        return y;
    }
}
impl Display for StatData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "sum: {}\nsize: {}\navg: {:.6}\nmin: {}\nmax: {}\n\u{3c3}: {:.6}\ns: {:.6}",
            self.sum, self.count, self.avg, self.min, self.max, self.sigma, self.standard_deviation
        )
    }
}

trait StatCalc {
    fn sum(self) -> f64;
    fn average(self) -> f64;
    fn min(self) -> f64;
    fn max(self) -> f64;
}

impl StatCalc for &Box<[f64]> {
    #[doc(hidden)]
    fn sum(self) -> f64 {
        //! Returns a sum value. If the result breaks the constraints
        //! of the type, should return None.
        //!
        //! Input: `Box<[f64]>`
        //! Output: `Option<f64>`
        let values = self;
        let mut result: f64 = 0.0;

        let mut i: usize = 0;

        while !result.is_nan() && i < values.len() {
            let inputted_num = values[i];

            result += inputted_num;
            i += 1
        }

        return result;
    }
    fn average(self) -> f64 {
        //! returns an average of the values--
        //! if the box is some how empty at this point, it returns none.
        let mut return_value = f64::NAN;
        let len = self.len();

        if len != 0 {
            return_value = self.sum() / len as f64
        }
        return return_value;
    }

    fn max(self) -> f64 {
        let mut relative_max = 0.0_f64;
        let mut i: usize = 0;
        let values = self;

        while i < values.len() {
            if i == 0 {
                relative_max = values[i]
            }
            if values[i] > relative_max {
                relative_max = values[i]
            }
            i += 1
        }
        return relative_max;
    }

    fn min(self) -> f64 {
        let mut relative_min = 0.0_f64;
        let mut i: usize = 0;
        let values = self;

        while i < values.len() {
            if i == 0 {
                relative_min = values[i]
            }
            if values[i] < relative_min {
                relative_min = values[i]
            }
            i += 1
        }

        return relative_min;
    }
}
trait InterpretData {
    fn sigma(self, values: &Box<[f64]>) -> f64;
    fn standard_deviation(self, values: &Box<[f64]>) -> f64;
}

impl InterpretData for StatData {
    fn sigma(self, values: &Box<[f64]>) -> f64 {
        let mut i: usize = 0;
        let len = self.count;
        let average = self.avg;

        let mut summative_res = 0_f64;

        while i < len {
            summative_res += (values[i] - average).powi(2);
            i += 1
        }
        summative_res = summative_res / len as f64;
        return summative_res.sqrt();
    }

    fn standard_deviation(self, values: &Box<[f64]>) -> f64 {
        let mut i: usize = 0;
        let len = self.count;
        let average = self.avg;

        let mut summative_res = 0_f64;

        while i < len {
            summative_res += (values[i] - average).powi(2);
            i += 1
        }
        summative_res = summative_res / (len - 1) as f64;
        return summative_res.sqrt();
    }
}

pub fn construction() {
    let _box = vec![5.0, 5.0, 3.0, 3.0, 6.5].into_boxed_slice();

    let x = StatData::new(_box);
    dbg!(x);
}

#[test]
fn test() {
    let _box = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_boxed_slice();
    let x = StatData::new(_box);
    dbg!(&x);
}
