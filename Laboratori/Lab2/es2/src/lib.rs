
pub mod solution {
    use std::ops::{Add, AddAssign};

    pub struct ComplexNumber {
        pub real: f64,
        pub imag: f64
    }

    impl ComplexNumber{

        pub fn new(real: f64, imag: f64) -> ComplexNumber {
            ComplexNumber{real, imag}
        }

        pub fn real(&self) -> f64 {
            self.real
        }

        pub fn imag(&self) -> f64 {
            self.imag
        }

        pub fn from_real(real: f64) -> ComplexNumber {
            ComplexNumber{real, imag: 0.0 }
        }

        pub fn to_tuple(&self) -> (f64, f64) {
            (self.real, self.imag)
        }

    }
    
    impl Add for ComplexNumber {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            ComplexNumber{real: self.real + rhs.real, imag: self.imag + rhs.imag}
        }
    }

    impl Add<f64> for ComplexNumber {
        type Output = Self;

        fn add(self, rhs: f64) -> Self::Output {
            ComplexNumber{real: self.real + rhs, imag: self.imag}
        }
    }

    impl Add<&ComplexNumber> for ComplexNumber {
        type Output = Self;

        fn add(self, rhs: &ComplexNumber) -> Self::Output {
            ComplexNumber{real: self.real + rhs.real, imag: self.imag + rhs.imag}
        }
    }

    impl Add for &ComplexNumber {
        type Output = ComplexNumber;

        fn add(self, rhs: &ComplexNumber) -> Self::Output {
            ComplexNumber{real: self.real + rhs.real, imag: self.imag + rhs.imag}
        }
    }

    impl AddAssign for ComplexNumber {

        fn add_assign(&mut self, rhs: ComplexNumber) {
            self.real = self.real + rhs.real;
            self.imag = self.imag + rhs.imag;
        }

    }

    impl Default for ComplexNumber {

        fn default() -> Self {
            ComplexNumber{real: 0.0, imag: 0.0}
        }
    }

    impl Into<f64> for ComplexNumber {

        fn into(self) -> f64 {

            if self.imag != 0.0 {
                panic!("Errore")
            }

            self.real
        }
    }



}
