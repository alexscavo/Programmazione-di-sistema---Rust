use std::collections::VecDeque;

pub mod solution {
    use std::cmp::Ordering;
    use std::hash::{Hash, Hasher};
    use std::ops::{Add, AddAssign};
    use std::error::Error;


    #[derive(Copy, Clone, Debug, PartialEq)]
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

        pub fn modulus(&self)->f64{
            ((self.real * self.real) + (self.imag*self.imag)).sqrt()
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

    /*
    impl Into<f64> for ComplexNumber {

        fn into(self) -> f64 {

            if self.imag != 0.0 {
                panic!("Errore")
            }

            self.real
        }
    }*/


    impl TryInto<f64> for ComplexNumber {

        type Error = String;

        fn try_into(self) -> Result<f64, Self::Error> {
            if self.imag != 0.0 {
                return Err("Errore".to_string());
            }
            else{
                Ok(self.real)
            }

        }
    }


    impl Into<ComplexNumber> for f64 {

        fn into(self) -> ComplexNumber {
            ComplexNumber::new(self, 0.0)
        }

    }

    impl PartialOrd for ComplexNumber {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

            if self.modulus() > other.modulus() {
                Some(Ordering::Greater)
            }
            else if self.modulus() < other.modulus(){
                Some(Ordering::Less)
            }
            else if self.modulus() == other.modulus() {
                Some(Ordering::Equal)
            }
            else{
                None
            }
        }

        fn lt(&self, other: &Self) -> bool {
            self.modulus() < other.modulus()
        }

        fn le(&self, other: &Self) -> bool {
            self.modulus() < other.modulus() || self.modulus().total_cmp(&other.modulus()) == Ordering::Equal
        }

        fn gt(&self, other: &Self) -> bool {
            self.modulus() > other.modulus()
        }

        fn ge(&self, other: &Self) -> bool {
            self.modulus() > other.modulus() || self.modulus().total_cmp(&other.modulus()) == Ordering::Equal
        }
    }

    impl Eq for ComplexNumber {

    }

    impl Ord for ComplexNumber {
        fn cmp(&self, other: &Self) -> Ordering {
            self.modulus().total_cmp(&other.modulus())
        }

    }

    impl AsRef<f64> for ComplexNumber {
        fn as_ref(&self) -> &f64 {
            &self.real
        }
    }

    impl AsMut<f64> for ComplexNumber {
        fn as_mut(&mut self) -> &mut f64 {
            &mut self.real
        }
    }

    impl Hash for ComplexNumber {
        fn hash<H: Hasher>(&self, state: &mut H) {
            (self.real+self.imag).to_bits().hash(state)
        }
    }
}
