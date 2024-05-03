pub mod solution {

    pub struct CircularBuffer<T> { //struct che rappresenta il buffer circolare
        pub buffer: Vec<T>,
        pub head: usize,
        pub tail: usize,
        pub capacity: usize,
        pub elem_number: u32
    }

    impl<T> CircularBuffer<T> where T: Default{

        pub fn new(cap: usize) -> Self {
            let mut cbuf: CircularBuffer<T> = CircularBuffer {
                buffer: Vec::with_capacity(cap),
                head: 0,
                tail: 0,
                capacity: cap,
                elem_number: 0
            };

            for i in 0..cap {
                cbuf.buffer.push(T::default());
            }

            cbuf
        }

        pub fn write(&mut self, item: T) -> Result<String, String> {

            if self.is_full() {
                return Err("Errore".to_string())
            }

            self.buffer[self.tail] = item;

            update_tail(self);
            self.elem_number = self.elem_number + 1;

            Ok("Ok".to_string())
        }

        pub fn update_tail(&mut self) -> usize{

            self.tail = self.tail + 1;

            if self.tail >= self.capacity  {
                self.tail = 0
            }

            self.tail
        }

        pub fn is_full(&self) -> bool {

            if self.elem_number == self.capacity as u32 {
                return true
            }

            false
        }
    }

    fn update_tail<T>(cbuf: &mut CircularBuffer<T>) -> usize{

        cbuf.tail = cbuf.tail + 1;

        if cbuf.tail >= cbuf.capacity {
            cbuf.tail = 0
        }

        cbuf.tail
    }

    fn is_full<T>(cbuf: &CircularBuffer<T>) -> bool {

        false
    }

}