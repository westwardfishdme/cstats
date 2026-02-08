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

    // quartiles
    pub q1: f64,
    pub q3: f64,
    pub iqr: f64,
    // count
    pub count: usize,
}

impl StatData {
    pub fn new(mut values: Box<[f64]>) -> StatData {
        values.sort_by(f64::total_cmp);
        let mut stat_struct = StatData {
            sum: values.sum(),
            avg: values.average(),
            min: values.min(),
            max: values.max(),
            median: values.median(),

            sigma: 0_f64,
            standard_deviation: 0_f64,
            //q1-q3
            q1: values.find_q1(),
            q3: values.find_q3(),
            iqr: 0_f64,
            count: values.len(),
        };
        // find the standard deviation as a sample, and as a population
        stat_struct.sigma = stat_struct.sigma(&values);
        stat_struct.standard_deviation = stat_struct.standard_deviation(&values);

        // find the interquartile range.
        stat_struct.iqr = stat_struct.iqr();

        return stat_struct;
    }
}
impl Display for StatData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "sum: {}
            \nsize: {} 
            \nmedian: {:.6}
            \navg: {:.6}
            \nmin: {}
            \nmax: {}
            \n\u{3c3}: {:.6}
            \ns: {:.6}
            \n
            \nQuartiles:
            \nQ1: {:.6}
            \nQ2: {:.6}
            \nQ3: {:.6}
            \nIQR: {:.6}",
            self.sum,
            self.count,
            self.median,
            self.avg,
            self.min,
            self.max,
            // standard deviation
            self.sigma,
            self.standard_deviation,
            // quatile stuff
            self.q1,
            self.median, // median == q2
            self.q3,
            self.iqr
        )
    }
}

trait StatCalc {
    fn sum(self) -> f64;
    fn average(self) -> f64;
    fn min(self) -> f64;
    fn max(self) -> f64;
    fn median(self) -> f64;
    fn find_q1(self) -> f64;
    fn find_q3(self) -> f64;
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
        let lastidx = self.len() - 1;
        return self[lastidx];
    }

    fn min(self) -> f64 {
        return self[0];
    }
    fn median(self) -> f64 {
        //! calculates the midpoint value. If the list
        //! is an uneven length, then return
        //! the point between 2 closest indices.
        let len = self.len();
        // subtract 1 for 0 point indexing
        let midpoint = (len / 2) - 1;
        let result: f64;

        // calculate the median based on midpoint values
        if len % 2 == 0 {
            let second_midpoint = midpoint + 1;
            result = (self[second_midpoint] + self[midpoint]) / 2.0;
        } else {
            result = self[midpoint];
        }

        return result;
    }
    fn find_q1(self) -> f64 {
        let len = self.len();
        let midpoint = (len / 2) - 1;
        let q1pt_idx0 = midpoint / 2;
        let result: f64;
        if len % 2 == 0 {
            let q1pt_idx1 = q1pt_idx0 + 1;
            result = (self[q1pt_idx0] + self[q1pt_idx1]) / 2.0;
        } else {
            result = self[q1pt_idx0];
        }
        return result;
    }
    fn find_q3(self) -> f64 {
        let len = self.len();
        let midpoint = (len / 2) - 1;
        let q1pt_idx0 = midpoint / 2;
        let q3pt_idx0 = q1pt_idx0 + midpoint + 1;
        let result: f64;
        if len % 2 == 0 {
            let q3pt_idx1 = q3pt_idx0 + 1;
            result = (self[q3pt_idx0] + self[q3pt_idx1]) / 2.0;
        } else {
            result = self[q3pt_idx0];
        }
        return result;
    }
}
trait InterpretData {
    fn sigma(self, values: &Box<[f64]>) -> f64;
    fn standard_deviation(self, values: &Box<[f64]>) -> f64;
    fn iqr(self) -> f64;
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
    fn iqr(self) -> f64 {
        self.q3 - self.q1
    }
}

#[test]
fn test() {
    // tests construction and data parsing.
    let _box = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_boxed_slice();
    let x = StatData::new(_box);
    dbg!(&x);
}
