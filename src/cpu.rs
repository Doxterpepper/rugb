// Number of registers
const REGISTER_COUNT: usize = 22; // Some registers are doubled up

pub struct Cpu {
    // registers
    sp: u16,   // stack pointer
    pc: usize, // program counter
    clock_ticks: usize,
    registers: [u8; REGISTER_COUNT],
    memory: Vec<u8>,
    ramstart: usize,
}

impl Cpu {
    /// Creates a new CPU with meory and ram position
    pub fn new(memory: Vec<u8>, ramstart: usize) -> Self {
        assert!(
            memory.len() > ramstart,
            format!(
                "ramstart cannot be less than memory size,\nmemory length: {0}\nramstart: {1}",
                memory.len(),
                ramstart
            )
        );

        Cpu {
            pc: 0x100, // Gameboy specific value
            clock_ticks: 0,
            sp: 0xfffe, // Gameboy specific value
            registers: [0; REGISTER_COUNT],
            memory: memory,
            ramstart: ramstart,
        }
    }

    /// Run the program in memory by doing binary translation
    pub fn execute(mut self) {
        loop {
            let mc = self.fetch();
            let reg = ((mc >> 3) & 0x07) as usize;

            match mc {
                0x01 | 0x11 | 0x21 => {
                    // LD commands
                    self.registers[reg + 1] = self.fetch();
                    self.registers[reg] = self.fetch();
                    println!(
                        "LD {0}{1}, 0x{2}{3}",
                        get_reg_name(reg),
                        get_reg_name(reg + 1),
                        self.registers[reg + 1],
                        self.registers[reg]
                    );
                    self.clock_ticks += 1;
                }
                _ => {
                    println!("NOP");
                    self.clock_ticks += 1 // Nop
                }
            }

            if self.pc > self.memory.len() {
                break;
            }
        }
    }

    /// fetch an instruction from memory
    fn fetch(&mut self) -> u8 {
        // Guard
        assert!(
            self.pc <= self.ramstart,
            format!("Invalid access to ram! fetch() {}", self.pc)
        );

        let value = self.memory[self.pc];
        self.pc += 1;
        value
    }
}

/// Convert a register name from integer to string for reading
fn get_reg_name(register: usize) -> String {
    let name = match register {
        0 => "B",
        1 => "C",
        2 => "D",
        3 => "E",
        4 => "H",
        5 => "L",
        7 => "A",
        _ => "U",
    };
    String::from(name)
}
