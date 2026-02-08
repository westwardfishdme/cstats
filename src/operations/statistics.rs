use std::fmt::{self, Display};

#[derive(Debug, Copy, Clone)]
#[doc()]
pub struct StatData {
    /// Datastructure for holding values related to
    /// simple statistical analysis. Organized by size
    /// in the latest git commit for memory optimizations
    /// related to size padding.
    // simple stats values
    pub sum: f64,
    pub avg: f64,

    // range values
    pub min: f64,
    pub max: f64,
    pub median: f64,

    // standard deviation values
    pub sigma: f64,              // population based
    pub standard_deviation: f64, // sample values
    // count
    pub count: usize,
}

impl StatData {
    pub fn new(values: Box<[f64]>) -> StatData {
        let x = StatData {
            sum: values.sum(),
            avg: values.average(),
            min: values.min(),
            max: values.max(),
            median: values.median(),

            sigma: 0_f64,
            standard_deviation: 0_f64,
            count: values.len(),
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
            "sum: {}\nsize: {} \nmedian: {:.6}\navg: {:.6}\nmin: {}\nmax: {}\n\u{3c3}: {:.6}\ns: {:.6}",
            self.sum,
            self.count,
            self.median,
            self.avg,
            self.min,
            self.max,
            self.sigma,
            self.standard_deviation
        )
    }
}

trait StatCalc {
    fn sum(self) -> f64;
    fn average(self) -> f64;
    fn min(self) -> f64;
    fn max(self) -> f64;
    fn median(self) -> f64;
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
        //! Returns an average of the values-- if the box is somehow empty at this point,
        //! it returns none.
        let mut return_value = f64::NAN;
        let len = self.len();

        if len != 0 {
            return_value = self.sum() / len as f64
        }
        return return_value;
    }

    fn max(self) -> f64 {
        //! Calculates the max value for a dataset
        //! Runs in O(n)
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
        //! Calculates the min value for a dataset
        //! Runs in O(n)
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
    fn median(self) -> f64 {
        //! sorts the list and calculates
        //! the midpoint value. If the list
        //! is an uneven length, then return
        //! the point between 2 closest indices.
        let mut list = self.clone();
        let len = list.len();
        // subtract 1 for 0 point indexing
        let midpoint = (len / 2) - 1;
        let result: f64;

        list.sort_by(f64::total_cmp);

        // calculate the median based on midpoint values
        if len % 2 == 0 {
            let second_midpoint = midpoint + 1;
            result = (list[second_midpoint] + list[midpoint]) / 2.0;
        } else {
            result = list[midpoint];
        }

        return result;
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

#[test]
fn test() {
    // tests construction and data parsing.
    let _box = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_boxed_slice();
    let x = StatData::new(_box);
    dbg!(&x);
}
