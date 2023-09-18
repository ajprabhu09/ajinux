use crate::devices::port::Port;

const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = PIC1_CMD + 1;

const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = PIC2_CMD + 1;

const PIC_EOI: u8 = 0x20;

const ICW1_ICW4: u8 = 0x01; /* Indicates that ICW4 will be present */
const ICW1_SINGLE: u8 = 0x02; /* Single (cascade) mode */
const ICW1_INTERVAL4: u8 = 0x04; /* Call address interval 4 (8) */
const ICW1_LEVEL: u8 = 0x08; /* Level triggered (edge) mode */
const ICW1_INIT: u8 = 0x10; /* Initialization - required! */
const ICW4_8086: u8 = 0x01; /* 8086/88 (MCS-80/85) mode */
const ICW4_AUTO: u8 = 0x02; /* Auto (normal) EOI */
const ICW4_BUF_SLAVE: u8 = 0x08; /* Buffered mode/slave */
const ICW4_BUF_MASTER: u8 = 0x0C; /* Buffered mode/master */
const ICW4_SFNM: u8 = 0x10; /* Special fully nested (not) */
pub struct Pic8259 {
    pic1_cmd: Port,
    pic1_data: Port,
    pic2_cmd: Port,
    pic2_data: Port,
}

impl Pic8259 {
    pub const fn new() -> Self {
        Self {
            pic1_cmd: Port(PIC1_CMD),
            pic1_data: Port(PIC1_DATA),
            pic2_cmd: Port(PIC2_CMD),
            pic2_data: Port(PIC2_DATA),
        }
    }

    pub fn eoi(&self, irq: u8) {
        if irq >= 8 {
            self.pic2_cmd.send_byte(PIC_EOI);
        }
        self.pic1_cmd.send_byte(PIC_EOI);
    }
    pub fn remap(&self, offset1: u8, offset2: u8) {
        let masks = (self.pic1_data.read_byte(), self.pic2_data.read_byte());

        self.pic1_cmd.send_byte(ICW1_INIT | ICW1_ICW4);
        self.pic2_cmd.send_byte(ICW1_INIT | ICW1_ICW4);
        self.pic1_data.send_byte(offset1);
        self.pic2_data.send_byte(offset2);
        self.pic1_data.send_byte(1 << 2); // set as master
        self.pic2_data.send_byte(1 << 1); // set as slave
        self.pic1_data.send_byte(ICW4_8086);
        self.pic2_data.send_byte(ICW4_8086);

        // restore masks
        self.pic1_data.send_byte(masks.0);
        self.pic2_data.send_byte(masks.1);
    }

    pub fn disable(&self) {
        self.pic1_data.send_byte(0xff);
        self.pic2_data.send_byte(0xff);
    }

    pub fn clear_irq(&self, line: u8) {
        if line < 8 {
            let curr_mask = self.pic1_data.read_byte();
            let removed = curr_mask & !(1 << line);
            self.pic1_data.send_byte(removed);
        } else {
            let line = line - 8;
            let curr_mask = self.pic2_data.read_byte();
            let removed = curr_mask & !(1 << line);
            self.pic2_data.send_byte(removed);
        }
    }
}
