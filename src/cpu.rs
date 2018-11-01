pub struct Cpu {
    // registers
    sp: u16,
    pc: usize,
    clock_ticks: usize,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: 0x100,
            clock_ticks: 0,
            sp: 0xfffe,
        }
    }

    pub fn run(&mut self, prog: Vec<u8>) {
        while self.pc < prog.len() {
            let instruction = match prog[self.pc] {
                0x00 => {
                    self.nop();
                    String::from("NOP")
                }
                _ => {
                    self.nop();
                    String::from("unknown")
                }
            };

            println!(
                "{0}\t{1:x}\t{2}",
                self.clock_ticks, prog[self.pc], instruction
            );

            self.clock_ticks += 1;
        }
    }

    fn nop(&mut self) {
        self.pc += 1;
    }
}
