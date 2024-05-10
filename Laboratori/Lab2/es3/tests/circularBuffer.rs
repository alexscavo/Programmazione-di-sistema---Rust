use es3::solution::CircularBuffer;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_new() {
        let buf: CircularBuffer<i32> = CircularBuffer::new(5);

        assert_eq!(buf.head, 0);
        assert_eq!(buf.tail, 0);
        assert_eq!(buf.buffer, [0, 0, 0, 0, 0]);
    }

    #[test]
    fn check_write() {
        let mut buf: CircularBuffer<i32> = CircularBuffer::new(5);

        let result = buf.write(1);
        assert_eq!(result, Ok("Ok".to_string()));

        for i in 1..5 {
            let result = buf.write(i);
        }

        let result = buf.write(9);
        assert_eq!(result, Err("Errore".to_string()));
    }
}