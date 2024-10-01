use std::path::PathBuf;

#[derive(Debug, Clone)]
struct ElfHeader {
    magic: [u8; 4],
    class: u8, // 1 -> 32 bits, 2 -> 64 bits
    data: u8, // 1 // little endian, 2 -> big endian
    version: u8,
    os_abi: u8,
    abi_version: u8,
    e_type: [u8; 2],
    e_machine: [u8; 2],
    e_version: [u8; 4],
    e_entry: [u8; 8], // entry point mem addr, 4 byte for 32 bits, 8 bytes for 64 bits
    e_phoff: [u8; 8], // start program header table
    e_shoff: [u8; 8], // start section header table
    e_flags: [u8; 4],
    // Program header info
    e_ehsize: [u8; 2],
    e_phentsize: [u8; 2],
    e_phnum: [u8; 2],
    // Section header info
    e_shentsize: [u8; 2],
    e_shnum: [u8; 2],
    e_shstrndx: [u8; 2],
}

#[derive(Debug, Clone)]
struct ElfProgramHeader {
    p_type: [u8; 4],
    p_flags: [u8; 4],
    p_offset: [u8; 8],
    p_vaddr: [u8; 8],
    p_paddr: [u8; 8],
    p_filesz: [u8; 8],
    p_memsz: [u8; 8],
    p_align: [u8; 8],
}

#[derive(Debug, Clone)]
struct ElfSection {
    name: [u8; 4],
    header_type: [u8; 4],
    flags: [u8; 8],
    addr: [u8; 8],
    offset: [u8; 8],
    size: [u8; 8],
    link: [u8; 4],
    info: [u8; 4],
    addr_align: [u8; 8],
    ent_size: [u8; 8],
}

impl TryFrom<&[u8]> for ElfProgramHeader {
    type Error = std::io::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let buffer = value;
        Ok(Self {
            p_type:    buffer_get::<4>(buffer, 0x00)?,
            p_flags:   buffer_get::<4>(buffer, 0x04)?,
            p_offset:  buffer_get::<8>(buffer, 0x08)?,
            p_vaddr:   buffer_get::<8>(buffer, 0x10)?,
            p_paddr:   buffer_get::<8>(buffer, 0x18)?,
            p_filesz:  buffer_get::<8>(buffer, 0x20)?,
            p_memsz:   buffer_get::<8>(buffer, 0x28)?,
            p_align:   buffer_get::<8>(buffer, 0x30)?,
        })
    }
}

impl TryFrom<&[u8]> for ElfSection {
    type Error = std::io::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {

        let buffer = value;

        Ok(Self {
            name: buffer_get::<4>(buffer, 0x0)?,
            header_type: buffer_get::<4>(buffer, 0x04)?,
            flags: buffer_get::<8>(buffer, 0x08)?,
            addr: buffer_get::<8>(buffer, 0x10)?,
            offset: buffer_get::<8>(buffer, 0x18)?,
            size: buffer_get::<8>(buffer, 0x20)?,
            link: buffer_get::<4>(buffer, 0x28)?,
            info: buffer_get::<4>(buffer, 0x2C)?,
            addr_align: buffer_get::<8>(buffer, 0x30)?,
            ent_size: buffer_get::<8>(buffer, 0x38)?,
        })
    }
}

fn buffer_get<const N: usize>(buffer: &[u8], offset: usize) -> Result<[u8; N], std::io::Error> {

    Ok(buffer
        .get(offset..offset+N)
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, ""))?
        .try_into()
        .unwrap()
    )
}

impl TryFrom<&[u8]> for ElfHeader {

    type Error = std::io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {

        // https://en.wikipedia.org/wiki/Executable_and_Linkable_Format#ELF_header
        let buffer = value;

        Ok(Self {
            magic:          buffer_get::<4>(buffer, 0x0)?,
            class:          buffer_get::<1>(buffer, 0x04)?[0],
            data:           buffer_get::<1>(buffer, 0x05)?[0],
            version:        buffer_get::<1>(buffer, 0x06)?[0],
            os_abi:         buffer_get::<1>(buffer, 0x07)?[0],
            abi_version:    buffer_get::<1>(buffer, 0x08)?[0],
            e_type:         buffer_get::<2>(buffer, 0x10)?,
            e_machine:      buffer_get::<2>(buffer, 0x12)?,
            e_version:      buffer_get::<4>(buffer, 0x14)?,
            e_entry:        buffer_get::<8>(buffer, 0x18)?,
            e_phoff:        buffer_get::<8>(buffer, 0x20)?,
            e_shoff:        buffer_get::<8>(buffer, 0x28)?,
            e_flags:        buffer_get::<4>(buffer, 0x30)?,
            e_ehsize:       buffer_get::<2>(buffer, 0x34)?,
            e_phentsize:    buffer_get::<2>(buffer, 0x36)?,
            e_phnum:        buffer_get::<2>(buffer, 0x38)?,
            e_shentsize:    buffer_get::<2>(buffer, 0x3A)?,
            e_shnum:        buffer_get::<2>(buffer, 0x3C)?,
            e_shstrndx:     buffer_get::<2>(buffer, 0x3E)?,
        })
    }
}

fn main() {

    let f = PathBuf::from("./basic");

    if !f.exists() {
        panic!("Please compile basic.s first !");
    }

    let elf_content = std::fs::read(f).unwrap();

    println!("elf content size: {:?}", elf_content.len());

    let elf_header = ElfHeader::try_from(elf_content.as_slice()).unwrap();

    println!("elf header: {:?}", elf_header);

    println!("elf header - program header offset: {:?}", u64::from_le_bytes(elf_header.e_phoff));
    println!("elf header - section header offset: {:?}", u64::from_le_bytes(elf_header.e_shoff));

    let program_header_count = u16::from_le_bytes(elf_header.e_shnum);
    let mut ph_offset = u64::from_le_bytes(elf_header.e_phoff) as usize;
    for i in 0..program_header_count {
        println!("Loading elf program header at offset: {}", ph_offset);
        let elf_program_header = ElfProgramHeader::try_from(&elf_content[ph_offset..]).unwrap();
        // println!("[{}] elf ph: {:?}", i, elf_program_header);
        // println!("type: {:x}", u32::from_le_bytes(elf_program_header.p_type));

        ph_offset += u16::from_le_bytes(elf_header.e_phentsize) as usize;
    }

    let mut sh_offset = u64::from_le_bytes(elf_header.e_shoff) as usize;
    println!("sh_offset: {}", sh_offset);
    let section_header_count = u16::from_le_bytes(elf_header.e_shnum);
    println!("section header count: {}", section_header_count);

    let mut datas = vec![];

    for i in 0..section_header_count {
        println!("Loading elf section at offset: {}", sh_offset);
        let elf_section = ElfSection::try_from(&elf_content[sh_offset..]).unwrap();
        // println!("[{}] elf section: {:?}", i, elf_section);
        // FIXME: Not the right offset
        // sh_offset += u64::from_le_bytes(elf_section.ent_size) as usize;

        if u32::from_le_bytes(elf_section.header_type) == 0x01 {
            let section_start = u64::from_le_bytes(elf_section.offset) as usize;
            let section_end = section_start + u64::from_le_bytes(elf_section.size) as usize;
            let data = elf_content[section_start..section_end].to_vec();
            // println!("data: {:?}", data);

            datas.push(data);
        }

        // Note: do not use elf_section.ent_size but rather elf_header shentsize
        sh_offset += u16::from_le_bytes(elf_header.e_shentsize) as usize;
    }

    //
    println!("data 0:");
    let data0 = &datas[0];
    let mut offset = 0;

    // dump instructions
    loop {
        let instruction_ = buffer_get::<4>(&data0, offset);
        if instruction_.is_err() {
            break;
        }
        let instruction = instruction_.unwrap();
        // println!("instruction: {:?}", instruction);
        offset += 4;

        // https://www.cs.sfu.ca/~ashriram/Courses/CS295/assets/notebooks/RISCV/RISCV_CARD.pdf

        // let opcode = u32::from_le_bytes(instruction);
        // let opcode = instruction[0] & 0b1111_1110;
        let opcode = instruction[0] & 0b0111_1111;

        let inst = u32::from_le_bytes(instruction);

        // println!("opcode: {:?}", opcode);
        // println!("opcode: {:#07b}", opcode);
        match opcode {
            0b0110011=> {
                println!("R inst");

                // load rd
                let rd_ = (inst >> 7) & u32::from_be_bytes([0, 0, 0, 0b0001_1111]);   // extract_bits(inst, 7, 11);
                let rs1 = (inst >> 15) & u32::from_be_bytes([0, 0, 0, 0b0000_1111]); //  extract_bits(inst, 15, 19);
                let rs2 = (inst >> 20) & u32::from_be_bytes([0, 0, 0, 0b0000_1111]);  // extract_bits(inst, 20, 24);

                println!("rd: {}, rs1: {}, rs2: {}", rd_, rs1, rs2);

                // load func3

            },
            0b0010011=> {
                // println!("I inst");
                // println!("I inst: {:#032b}", inst);

                // 0b000000110110010000 10011
                // 0b000000000000011011001000010011
                // 0b000000000000011011001000010011,

                // 0b101000 10010 000 10010 0010011

                let func3 = (inst >> 12) & u32::from_be_bytes([0, 0, 0, 0b0000_0011]);
                // println!("func3: {} - {}", func3, instruction_funct_name(opcode, func3));

                let func_name = match func3 {
                    0x0 => "addi",
                    0x01 => "slli",
                    _ => panic!("Unknown func name for func3: {}", func3),
                };
                // println!("func3: {} - {}", func3, func_name);

                // load rd
                let rd_mask = u32::from_be_bytes([0, 0, 0, 0b0001_1111]);
                // println!("  inst: {:#032b}, rd_mask: {:#032b}, res: {:#032b}", inst >> 7, rd_mask, (inst >> 7) & rd_mask);
                let rd_ = (inst >> 7) & rd_mask;   // extract_bits(inst, 7, 11);
                let rs1 = (inst >> 15) & u32::from_be_bytes([0, 0, 0, 0b0001_1111]); //  extract_bits(inst, 15, 19);
                let imm = (inst >> 20) & u32::from_be_bytes([0, 0, 0b0000_1111, 0b1111_1111]);  // extract_bits(inst, 20, 24);

                println!("[I] func: {}, rd: {}, rs1: {}, imm: {}", func_name, rd_, rs1, imm);
            },
            0b0000011=> {
                // println!("I (load) inst");

                let func3 = (inst >> 12) & u32::from_be_bytes([0, 0, 0, 0b0000_0011]);

                let func_name = match func3 {
                    0x0 => "lb",
                    0x01 => "lh",
                    0x02 => "lw",
                    0x04 => "lbu",
                    0x05 => "lhu",
                    _ => panic!("[I (load)] Unknown func name for func3: {}", func3),
                };
                // println!("func3: {} - {}", func3, func_name);

                // load rd

                let rd_ = (inst >> 7) & u32::from_be_bytes([0, 0, 0, 0b0001_1111]);   // extract_bits(inst, 7, 11);
                let rs1 = (inst >> 15) & u32::from_be_bytes([0, 0, 0, 0b0001_1111]); //  extract_bits(inst, 15, 19);
                println!("[I load] func: {}, rd: {}, rs1: {}", func_name, rd_, rs1);
            }
            0b0100011 => {

                let func3 = (inst >> 12) & u32::from_be_bytes([0, 0, 0, 0b0000_0011]);
                let func_name = match func3 {
                    0x0 => "sb",
                    0x01 => "sh",
                    0x02 => "sw",
                    _ => panic!("[S] Unknown func name for func3: {}", func3),
                };
                println!("[S] func: {}", func_name);
            },
            0b1100011 => {

                // 11111111 00111001 01001000 11100011
                //                               opcode
                // 11111111 00111001 01001000 11 100011

                // println!("B inst: {:32b}", inst);
                // println!("B inst: {:#032b}", inst);
                // let func3 = (inst >> 12) & u32::from_be_bytes([0, 0, 0, 0b0000_0011]);
                // println!("func3: {}", func3);
                // // let rd_ = (inst >> 7) & u32::from_le_bytes([0, 0, 0, 0b0001_1111]);   // extract_bits(inst, 7, 11);
                // let rs1 = (inst >> 15) & u32::from_be_bytes([0, 0, 0, 0b0000_1111]); //  extract_bits(inst, 15, 19);
                // let rs2 = (inst >> 20) & u32::from_be_bytes([0, 0, 0, 0b0000_1111]);  // extract_bits(inst, 20, 24);
                // println!("rs1: {}, rs2: {}", rs1, rs2);

                let func3 = (inst >> 12) & u32::from_be_bytes([0, 0, 0, 0b0000_0011]);
                let func_name = match func3 {
                    0x0 => "beq",
                    0x01 => "bne",
                    0x04 => "blt",
                    0x05 => "bge",
                    0x06 => "bltu",
                    0x07 => "bgeu",
                    _ => panic!("[B] Unknown func name for func3: {}", func3),
                };
                println!("[B] {}, rd: {}, imm: {}", func_name, "", "");
            }
            0b1101111 => {
                println!("[J] jal");
            },
            0b1100111 => {
                println!("[I] jalr");
            },
            0b0010111 => {
                let rd_ = (inst >> 7) & u32::from_be_bytes([0, 0, 0, 0b0001_1111]);   // extract_bits(inst, 7, 11);
                let imm = (inst >> 12) & u32::from_be_bytes([0, 0b0000_0111, 0b1111_1111, 0b1111_1111]);  // extract_bits(inst, 20, 24);
                println!("[U] auipc, rd: {}, imm: {}", rd_, imm);
            },
            0b0110111 => {
                
                // println!("[U] lui inst: {:#032b}", inst);
                
                let rd_ = (inst >> 7) & u32::from_be_bytes([0, 0, 0, 0b0001_1111]);
                let imm_mask = u32::from_be_bytes([0, 0b0000_0111, 0b1111_1111, 0b1111_1111]);
                let imm = (inst >> 12) & imm_mask;
                println!("[U] lui, rd: {}, imm: {:#x}", rd_, imm);
            },
            0b1110011 => {
                // println!("[ecall|ebreak|csrr]");
                println!("I inst: {:#032b}", inst);
                let rd_ = (inst >> 7) & u32::from_be_bytes([0, 0, 0, 0b0001_1111]);
                match rd_ {
                    0x00000 => {

                        // ecall / ebreak

                        match inst {
                            b00000000000000000000000001110011 => {
                                println!("[I] ecall");
                            },
                            b00010000010100000000000001110011 => {
                                println!("[I] wfi");
                            }
                            _ => {
                                println!("[I] unknown");
                            }
                        }

                        /*
                        let imm_mask = u32::from_be_bytes([0, 0b0000_0111, 0b1111_1111, 0b1111_1111]);
                        let imm = (inst >> 12) & imm_mask;

                        match imm {
                            b000000000000 => {
                                println!("ecall")
                            },
                            b000000000001 => {
                                println!("ebreak");
                            }
                            b000100000101 => {
                                println!("wfi");
                            }
                            _ => {
                                println!("Unknown inst");
                            }
                        }
                        */
                    },
                    _ => {
                        let func3 = (inst >> 12) & u32::from_be_bytes([0, 0, 0, 0b0000_0011]);
                        let func_name = match func3 {
                            b001 => "csrrw",
                            b010 => "csrrs",
                            b011 => "csrrc",
                            b101 => "csrrwi",
                            b110 => "csrrsi",
                            b111 => "csrrci",
                            _ => panic!("[B] Unknown func name for func3: {:03b}", func3),
                        };
                        println!("[I] {}", func_name);
                    }
                }
                // println!("[B] {}, rd: {}, imm: {}", func_name, "", "");
            }
            _ => {
                println!("Unknown opcode: {:b}", opcode);
            }
        }

    }

    // process instructions
    /*
    loop {
        let instruction_ = buffer_get::<4>(&data0, offset);
        if instruction_.is_err() {
            break;
        }
        let instruction = instruction_.unwrap();
        offset += 4;

        let opcode = instruction[0] & 0b0111_1111;
        let inst = u32::from_le_bytes(instruction);
        match opcode {
            _ => {
                todo!()
            }
        }
    }
    */
}
