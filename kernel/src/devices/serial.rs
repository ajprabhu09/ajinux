use super::{port::Port, vga::{ConsoleDisplay, ConsoleErrType, PackedColor}};


pub struct SerialCom {
    port: Port,
}

pub const COM1:u16 =	0x3F8;
pub const COM2:u16 =	0x2F8;
pub const COM3:u16 =	0x3E8;
pub const COM4:u16 =	0x2E8;
pub const COM5:u16 =	0x5F8;
pub const COM6:u16 =	0x4F8;
pub const COM7:u16 =	0x5E8;
pub const COM8:u16 =	0x4E8;

impl SerialCom {
    pub const fn new(port: u16) -> Self {
        Self { port: Port(port) }
    }
    pub fn connect(self) -> Result<Self, &'static str> {
        let port = self.port.0;
        Port(port + 1).send_byte(0x00);    // Disable all interrupts
        Port(port + 3).send_byte(0x80);    // Enable DLAB (set baud rate divisor)
        Port(port + 0).send_byte(0x03);    // Set divisor to 3 (lo byte) 38400 baud
        Port(port + 1).send_byte(0x00);    //                  (hi byte)
        Port(port + 3).send_byte(0x03);    // 8 bits, no parity, one stop bit
        Port(port + 2).send_byte(0xC7);    // Enable FIFO, clear them, with 14-byte threshold
        Port(port + 4).send_byte(0x0B);    // IRQs enabled, RTS/DSR set
        Port(port + 4).send_byte(0x1E);    // Set in loopback mode, test the serial chip
        Port(port + 0).send_byte(0xAE);    // Test serial chip (send byte 0xAE and check if serial returns same byte)
        
        if self.port.read_byte() != 0xAE {
            return Err("unable to setup serial port");
        }
        Port(port + 4).send_byte(0x0F);
        Ok(self)
    }

    fn serial_rcvd(&self) -> bool {
        let port = Port(self.port.0 + 5);
        return port.read_byte() & 0x01 == 1;
    }
    fn is_tx_empty(&self) -> bool {
        let port = Port(self.port.0 + 5);
        return port.read_byte() & 0x20 > 0;
    }

    fn read_char(&self) -> u8 {
        while !self.serial_rcvd() {}
        return self.port.read_byte();
    }
    fn write_char(&self, byt: u8) {
        while !self.is_tx_empty() {}
        return self.port.send_byte(byt);
    }

}

impl ConsoleDisplay for SerialCom {
    fn put_byte(&mut self, ch: u8) -> Result<(), ConsoleErrType> {
        self.write_char(ch);
        Ok(())
    }

    fn put_bytes(&mut self, ch: &[u8]) -> Result<(), ConsoleErrType> {
       for c in ch {
            self.put_byte(*c)?;
       }
       Ok(())
    }

    fn draw_char(&mut self, loc: (i32, i32), ch: u8, color: PackedColor) {
        panic!("setting color is not supported in serial")
    }

    fn get_char(&mut self, loc: (i32, i32)) -> Result<u8, ConsoleErrType> {
        panic!("getting chars at location is not supported in serial")
    }

    fn set_term_color(&mut self, color: PackedColor) {
        panic!("setting term color not supported in serial")
        
    }

    fn get_term_color(&mut self) -> Result<PackedColor, ConsoleErrType> {
        panic!("getting term color is not supported in serial")
    }

    fn set_cursor(&mut self, loc: (i32, i32)) -> Result<(), ConsoleErrType> {
        panic!("setting cursor is not supoorted in serial")
    }

    fn get_cursor(&mut self) -> (i32, i32) {
        panic!("getting cursor is not supoorted in serial")
    }

    fn hide_cursor(&mut self) {
        panic!("hiding cursor is not supoorted in serial")
    }

    fn clear(&mut self) {
        panic!("clearing cursor is not supoorted in serial")
    }
}