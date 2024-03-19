// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::error::RustQuantError;

use super::Distribution;
// use crate::math::DistributionError;
use num::Complex;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Bernoulli distribution: X ~ Bern(p)
pub struct Bernoulli {
    /// Probability of k = 1 (q = 1 - p: probability of k = 0).
    p: f64,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Default for Bernoulli {
    fn default() -> Self {
        Self::new(0.5)
    }
}

impl Bernoulli {
    /// New instance of a Bernoulli distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let bernoulli = Bernoulli::new(0.5);
    ///
    /// assert_eq!(bernoulli.mean(), 0.5);
    /// assert_approx_equal!(bernoulli.cf(1.0).re, 0.7701511, 1e-7);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `probability` is not between 0 and 1.
    #[must_use]
    pub fn new(probability: f64) -> Bernoulli {
        assert!((0.0..=1.0).contains(&probability));

        Bernoulli { p: probability }
    }
}

impl Distribution for Bernoulli {
    /// Characteristic function of the Bernoulli distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let bernoulli = Bernoulli::new(0.5);
    ///
    /// assert_approx_equal!(bernoulli.cf(1.0).re, 0.7701511, 1e-7);
    /// assert_approx_equal!(bernoulli.cf(1.0).im, 0.4207355, 1e-7);
    /// ```
    fn cf(&self, t: f64) -> Complex<f64> {
        assert!((0.0..=1.0).contains(&self.p));

        let i: Complex<f64> = Complex::i();
        1.0 - self.p + self.p * (i * t).exp()
    }

    /// Probability density function of the Bernoulli distribution.
    /// Using this method will call `self.pmf()` instead.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let bernoulli = Bernoulli::new(0.5);
    ///
    /// assert_eq!(bernoulli.pdf(1.0), 0.5);
    /// ```
    fn pdf(&self, x: f64) -> f64 {
        self.pmf(x)
    }

    /// Probability mass function of the Bernoulli distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let bernoulli = Bernoulli::new(0.5);
    ///
    /// assert_eq!(bernoulli.pmf(1.0), 0.5);
    /// ```
    fn pmf(&self, k: f64) -> f64 {
        assert!((0.0..=1.0).contains(&self.p));
        assert!(k == 0.0 || k == 1.0);

        (self.p).powi(k as i32) * (1.0 - self.p).powi(1 - k as i32)
    }

    /// Cumulative distribution function of the Bernoulli distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let bernoulli = Bernoulli::new(0.5);
    ///
    /// assert_eq!(bernoulli.cdf(0.0), 0.5);
    /// assert_eq!(bernoulli.cdf(1.0), 1.0);
    /// ```
    fn cdf(&self, k: f64) -> f64 {
        assert!((0.0..=1.0).contains(&self.p));

        if (k as i32) < 0 {
            0.0
        } else if (0..1).contains(&(k as i32)) {
            1.0 - self.p
        } else {
            1.0
        }
    }

    /// Inverse (quantile) distribution function of the Bernoulli distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let bernoulli = Bernoulli::new(0.5);
    ///
    /// assert_eq!(bernoulli.inv_cdf(0.5), 1.0);
    /// ```
    fn inv_cdf(&self, p: f64) -> f64 {
        assert!((0.0..=1.0).contains(&p));

        if p < 1.0 - self.p {
            0.0
        } else {
            1.0
        }
    }

    /// Mean of the Bernoulli distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let bernoulli = Bernoulli::new(0.5);
    ///
    /// assert_eq!(bernoulli.mean(), 0.5);
    /// ```
    fn mean(&self) -> f64 {
        self.p
    }

    /// Median of the Bernoulli distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let bernoulli = Bernoulli::new(0.5);
    ///
    /// assert_eq!(bernoulli.median(), 1.0);
    /// ```
    fn median(&self) -> f64 {
        if self.p < 0.5 {
            0.0
        } else {
            1.0
        }
    }

    /// Mode of the Bernoulli distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let bernoulli = Bernoulli::new(0.5);
    ///
    /// assert_eq!(bernoulli.mode(), 0.0);
    /// ```
    fn mode(&self) -> f64 {
        if self.p <= 0.5 {
            // if p == 0.5 both 0 and 1 are modes
            0.0
        } else {
            1.0
        }
    }

    /// Variance of the Bernoulli distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let bernoulli = Bernoulli::new(0.5);
    ///
    /// assert_eq!(bernoulli.variance(), 0.25);
    /// ```
    fn variance(&self) -> f64 {
        self.p * (1.0 - self.p)
    }

    /// Skewness of the Bernoulli distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let bernoulli = Bernoulli::new(0.5);
    ///
    /// assert_eq!(bernoulli.skewness(), 0.0);
    /// ```
    fn skewness(&self) -> f64 {
        let p = self.p;
        ((1.0 - p) - p) / (p * (1.0 - p)).sqrt()
    }

    /// Kurtosis of the Bernoulli distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::math::distributions::*;
    ///
    /// let bernoulli = Bernoulli::new(0.5);
    ///
    /// assert_eq!(bernoulli.kurtosis(), -2.0);
    /// ```
    fn kurtosis(&self) -> f64 {
        let p = self.p;
        (1.0 - 6.0 * p * (1.0 - p)) / (p * (1.0 - p))
    }

    /// Entropy of the Bernoulli distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let bernoulli = Bernoulli::new(0.5);
    ///
    /// assert_approx_equal!(bernoulli.entropy(), 0.6931472, 1e-7);
    /// ```
    fn entropy(&self) -> f64 {
        (self.p - 1.0) * (1.0 - self.p).ln() - self.p * (self.p).ln()
    }

    /// Moment generating function of the Bernoulli distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let bernoulli = Bernoulli::new(0.5);
    ///
    /// assert_approx_equal!(bernoulli.mgf(1.0), 1.8591409 , 1e-7);
    /// ```
    fn mgf(&self, t: f64) -> f64 {
        1.0 - self.p + self.p * f64::exp(t)
    }

    /// Generate random samples from a Bernoulli distribution.
    /// # Examples
    /// ```
    /// # use RustQuant::assert_approx_equal;
    /// # use RustQuant::math::distributions::*;
    ///
    /// let bernoulli = Bernoulli::new(0.5);
    ///
    /// let sample = bernoulli.sample(100).expect("Bernoulli sampled.");
    /// let mean = sample.iter().sum::<f64>() / sample.len() as f64;
    ///
    /// assert_approx_equal!(mean, bernoulli.mean(), 0.1);
    /// ```
    fn sample(&self, n: usize) -> Result<Vec<f64>, RustQuantError> {
        // IMPORT HERE TO AVOID CLASH WITH
        // `RustQuant::distributions::Distribution`
        use rand::thread_rng;
        use rand_distr::{Bernoulli, Distribution};

        assert!(n > 0);

        let mut rng = thread_rng();

        let dist = Bernoulli::new(self.p)?;

        let mut variates: Vec<f64> = Vec::with_capacity(n);

        for _ in 0..variates.capacity() {
            variates.push(usize::from(dist.sample(&mut rng)) as f64);
        }

        Ok(variates)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests_bernoulli {
    use super::*;
    use crate::assert_approx_equal;
    use crate::error::RustQuantError;
    use std::f64::EPSILON as EPS;

    #[test]
    fn test_bernoulli_functions() {
        let dist = Bernoulli::new(1.0);

        // Characteristic function
        let cf = dist.cf(1.0);
        assert_approx_equal!(cf.re, 0.540_302_305_868_139_8, EPS);
        assert_approx_equal!(cf.im, 0.841_470_984_807_896_5, EPS);

        let bernoulli = Bernoulli::new(0.5);

        // Probability mass function
        let pmf = dist.pmf(1.0);
        assert_approx_equal!(pmf, 1.0, EPS);
        // Test pmf for k = 0.0 and 1.0
        let pmf_zero = bernoulli.pmf(0.0);
        let pmf_one = bernoulli.pmf(1.0);
        assert_approx_equal!(pmf_zero, 0.5, EPS);
        assert_approx_equal!(pmf_one, 0.5, EPS);

        // Distribution function
        let cdf = dist.cdf(1.0);
        assert_approx_equal!(cdf, 1.0, EPS);
        // Test cdf for k = -1.0, 0.0, 0.5, 1.0 and 2.0
        let cdf_neg = bernoulli.cdf(-1.0);
        let cdf_zero = bernoulli.cdf(0.0);
        let cdf_half = bernoulli.cdf(0.5);
        let cdf_one = bernoulli.cdf(1.0);
        let cdf_two = bernoulli.cdf(2.0);
        assert_approx_equal!(cdf_neg, 0.0, EPS);
        assert_approx_equal!(cdf_zero, 0.5, EPS);
        assert_approx_equal!(cdf_half, 0.5, EPS);
        assert_approx_equal!(cdf_one, 1.0, EPS);
        assert_approx_equal!(cdf_two, 1.0, EPS);

        // Test moment generating function for t = 1.0
        let mgf = bernoulli.mgf(1.0);
        assert_approx_equal!(mgf, 1.0 - 0.5 + 0.5 * 1_f64.exp(), EPS);

        // Test characteristic function for t = 1.0
        let cf = bernoulli.cf(1.0);
        assert_eq!(
            cf,
            Complex::new(1.0 - 0.5 + 0.5 * 1_f64.cos(), 0.5 * 1_f64.sin())
        );
    }

    #[test]
    fn test_bernoulli_moments() {
        let bernoulli = Bernoulli::new(0.5);

        // Test mean and variance
        assert_approx_equal!(bernoulli.mean(), 0.5, EPS);
        assert_approx_equal!(bernoulli.variance(), 0.25, EPS);

        // Test skewness and kurtosis
        assert_approx_equal!(bernoulli.skewness(), 0.0, EPS);
        assert_approx_equal!(bernoulli.kurtosis(), -2.0, EPS);
    }

    #[test]
    fn test_bernoulli_entropy() {
        let bernoulli = Bernoulli::new(0.5);

        // Test entropy
        assert_approx_equal!(
            bernoulli.entropy(),
            -(0.5f64.ln() * 0.5 + (1.0 - 0.5_f64).ln() * (1.0 - 0.5)),
            EPS
        );
    }

    #[test]
    fn test_default() {
        let bernoulli = Bernoulli::default();
        assert_approx_equal!(bernoulli.p, 0.5, EPS);
    }

    #[test]
    #[should_panic(expected = "assertion failed: (0.0..=1.0).contains(&probability)")]
    fn test_new_invalid_probability_low() {
        let _ = Bernoulli::new(-0.5);
    }

    #[test]
    #[should_panic(expected = "assertion failed: (0.0..=1.0).contains(&probability)")]
    fn test_new_invalid_probability_high() {
        let _ = Bernoulli::new(1.5);
    }

    #[test]
    #[should_panic(expected = "assertion failed: k == 0.0 || k == 1.0")]
    fn test_pmf_invalid_input() {
        let bernoulli = Bernoulli::new(0.5);
        bernoulli.pmf(2.0);
    }

    #[test]
    fn test_cdf_negative_input() {
        let bernoulli = Bernoulli::new(0.5);
        let cdf_neg = bernoulli.cdf(-1.0);
        assert_approx_equal!(cdf_neg, 0.0, EPS);
    }

    #[test]
    fn test_cdf_positive_input() {
        let bernoulli = Bernoulli::new(0.5);
        let cdf_one = bernoulli.cdf(1.0);
        let cdf_two = bernoulli.cdf(2.0);
        assert_approx_equal!(cdf_one, 1.0, EPS);
        assert_approx_equal!(cdf_two, 1.0, EPS);
    }

    #[test]
    fn test_inv_cdf() {
        let bernoulli = Bernoulli::new(0.5);
        let inv_cdf_one = bernoulli.inv_cdf(0.5);
        let inv_cdf_two = bernoulli.inv_cdf(0.3);
        assert_approx_equal!(inv_cdf_one, 1.0, EPS);
        assert_approx_equal!(inv_cdf_two, 0.0, EPS);
    }

    #[test]
    fn test_median() {
        let bernoulli = Bernoulli::new(0.5);
        let median = bernoulli.median();
        assert_approx_equal!(median, 1.0, EPS);
    }

    #[test]
    fn test_mode() {
        let bernoulli = Bernoulli::new(0.5);
        let mode = bernoulli.mode();
        assert_approx_equal!(mode, 0.0, EPS);
    }

    #[test]
    #[should_panic(expected = "assertion failed: n > 0")]
    fn test_sample_zero_size() {
        let bernoulli = Bernoulli::new(0.5);
        let _ = bernoulli.sample(0);
    }

    #[test]
    fn test_sample_positive_size() -> Result<(), RustQuantError> {
        let bernoulli = Bernoulli::new(0.5);
        let sample = bernoulli.sample(100)?;
        assert_eq!(sample.len(), 100);
        for &value in &sample {
            assert!(value == 0.0 || (value - 1.0).abs() < EPS);
        }

        Ok(())
    }
}
