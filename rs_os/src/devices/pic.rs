use crate::utils::asm::outb;

const ADDR_PIC_BASE: u8 = 0x20;
const OFF_ICW: u16 = 0x00;
const OFF_OCW: u16 = 0x01;
const SIZE_PIC: u8 = 0x80;

const MASTER_ICW: u16 = (ADDR_PIC_BASE as u16 + OFF_ICW) as u16;
const MASTER_OCW: u16 = (ADDR_PIC_BASE as u16 + OFF_OCW) as u16;
const SLAVE_ICW: u16 = (ADDR_PIC_BASE as u16 + OFF_ICW + SIZE_PIC as u16) as u16;
const SLAVE_OCW: u16 = (ADDR_PIC_BASE as u16 + OFF_OCW + SIZE_PIC as u16) as u16;

/*
** The following banks of definitions ICW1, ICW2, ICW3, and ICW4 are used
** to define the fields of the various ICWs for initialisation of the PICs
*/

/*
**      ICW1
*/

const ICW_TEMPLATE: u8 = 0x10;

const LEVL_TRIGGER: u8 = 0x08;
const EDGE_TRIGGER: u8 = 0x00;
const ADDR_INTRVL4: u8 = 0x04;
const ADDR_INTRVL8: u8 = 0x00;
const SINGLE_MODE: u8 = 0x02;
const CASCADE_MODE: u8 = 0x00;
const ICW4__NEEDED: u8 = 0x01;
const NO_ICW4_NEED: u8 = 0x00;

/*
**      ICW2 is used to hold the interrupt base.
*/

/*
**      ICW3
*/

const SLAVE_ON_IR0: u8 = 0x01;
const SLAVE_ON_IR1: u8 = 0x02;
const SLAVE_ON_IR2: u8 = 0x04;
const SLAVE_ON_IR3: u8 = 0x08;
const SLAVE_ON_IR4: u8 = 0x10;
const SLAVE_ON_IR5: u8 = 0x20;
const SLAVE_ON_IR6: u8 = 0x40;
const SLAVE_ON_IR7: u8 = 0x80;

const I_AM_SLAVE_0: u8 = 0x00;
const I_AM_SLAVE_1: u8 = 0x01;
const I_AM_SLAVE_2: u8 = 0x02;
const I_AM_SLAVE_3: u8 = 0x03;
const I_AM_SLAVE_4: u8 = 0x04;
const I_AM_SLAVE_5: u8 = 0x05;
const I_AM_SLAVE_6: u8 = 0x06;
const I_AM_SLAVE_7: u8 = 0x07;

/*
**      ICW4
*/

const SNF_MODE_ENA: u8 = 0x10;
const SNF_MODE_DIS: u8 = 0x00;
const BUFSLAV_MODE: u8 = 0x08;
const BUFMSTR_MODE: u8 = 0x0C;
const NONBUFD_MODE: u8 = 0x00;
const AUTO_EOI_MOD: u8 = 0x02;
const NRML_EOI_MOD: u8 = 0x00;
const I8086_EMM_MOD: u8 = 0x01;
const SET_MCS_MODE: u8 = 0x00;

/*
**      OCW1
*/

const PICM_MASK: u8 = 0xFF;
const PICS_MASK: u8 = 0xFF;

/*
**      OCW2
*/

const NON_SPEC_EOI: u8 = 0x20;
const SPECIFIC_EOI: u8 = 0x60;
const ROT_NON_SPEC: u8 = 0xA0;
const SET_ROT_AEOI: u8 = 0x80;
const RSET_ROTAEOI: u8 = 0x00;
const ROT_SPEC_EOI: u8 = 0xE0;
const SET_PRIORITY: u8 = 0xC0;
const NO_OPERATION: u8 = 0x40;

const SEND_EOI_IR0: u8 = 0x00;
const SEND_EOI_IR1: u8 = 0x01;
const SEND_EOI_IR2: u8 = 0x02;
const SEND_EOI_IR3: u8 = 0x03;
const SEND_EOI_IR4: u8 = 0x04;
const SEND_EOI_IR5: u8 = 0x05;
const SEND_EOI_IR6: u8 = 0x06;
const SEND_EOI_IR7: u8 = 0x07;

/*
**      OCW3
*/

const OCW_TEMPLATE: u8 = 0x08;
const SPECIAL_MASK: u8 = 0x40;
const MASK_MDE_SET: u8 = 0x20;
const MASK_MDE_RST: u8 = 0x00;
const POLL_COMMAND: u8 = 0x04;
const NO_POLL_CMND: u8 = 0x00;
const READ_NEXT_RD: u8 = 0x02;
const READ_IR_ONRD: u8 = 0x00;
const READ_IS_ONRD: u8 = 0x01;

/*
**      Standard PIC initialization values for PCs.
*/
const PICM_ICW1: u8 = (ICW_TEMPLATE | EDGE_TRIGGER | ADDR_INTRVL8 | CASCADE_MODE | ICW4__NEEDED);
const PICM_ICW3: u8 = (SLAVE_ON_IR2);
const PICM_ICW4: u8 = (SNF_MODE_DIS | NONBUFD_MODE | NRML_EOI_MOD | I8086_EMM_MOD);

const PICS_ICW1: u8 = (ICW_TEMPLATE | EDGE_TRIGGER | ADDR_INTRVL8 | CASCADE_MODE | ICW4__NEEDED);
const PICS_ICW3: u8 = (I_AM_SLAVE_2);
const PICS_ICW4: u8 = (SNF_MODE_DIS | NONBUFD_MODE | NRML_EOI_MOD | I8086_EMM_MOD);

/** @brief Default location of the master PIC's interrupts in the IDT */
const X86_PIC_MASTER_IRQ_BASE: u8 = 0x20;
/** @brief Default location of the slave  PIC's interrupts in the IDT */
const X86_PIC_SLAVE_IRQ_BASE: u8 = 0x28;

pub fn pic_init(master_base: u8, slave_base: u8) {
    unsafe {
        outb(MASTER_ICW, PICM_ICW1);
        outb(MASTER_OCW, master_base);
        outb(MASTER_OCW, PICM_ICW3);
        outb(MASTER_OCW, PICM_ICW4);

        /* Same dance with the slave as with the master */
        outb(SLAVE_ICW, PICS_ICW1);
        outb(SLAVE_OCW, slave_base);
        outb(SLAVE_OCW, PICS_ICW3);
        outb(SLAVE_OCW, PICS_ICW4);

        /* Tell the master and slave that any IRQs they had outstanding
         * have been acknowledged.
         */
        outb(MASTER_ICW, NON_SPEC_EOI);
        outb(SLAVE_ICW, NON_SPEC_EOI);

        /* Enable all IRQs on master and slave */
        outb(SLAVE_OCW, 0);
        outb(MASTER_OCW, 0);
    }
}

pub fn pic_acknowledge(irq: u8) {
    unsafe {
        /* Note that SEND_EOI_IRn is just n, so we don't need any fancy map */
        if (irq <= 7) {
            outb(MASTER_ICW, SPECIFIC_EOI | irq);
        } else if (irq <= 15) {
            outb(SLAVE_ICW, SPECIFIC_EOI | (irq & 0x07));
            outb(MASTER_ICW, SPECIFIC_EOI | PICS_ICW3);
        } else {
            return;
        }
    }
}

/** @brief Acknowledge the master PIC for an arbitrary interrupt. */
pub fn pic_acknowledge_any_master() {
    unsafe {
        outb(MASTER_ICW, NON_SPEC_EOI);
    }
}

/** @brief Acknowledge the slave PIC for an arbitrary interrupt.
 *  @note Also acknowledges the master's input from the slave.
 */
pub fn pic_acknowledge_any_slave() {
    unsafe {
        outb(SLAVE_ICW, NON_SPEC_EOI);
        outb(MASTER_ICW, SPECIFIC_EOI | PICS_ICW3);
    }
}
