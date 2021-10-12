use std::env;
use std::result::Result;

use clap::{crate_version, App, Arg};
use riscv_decode::{
    decode,
    types::{IType, RType, SType, UType, ShiftType},
    Instruction,
};

fn print_u_type(instruction: Instruction, utype: UType) {
    print!("Instruction = {:?}\n", instruction);
    print!("Immediate = {:?}\n", utype.imm());
    print!("Destination register = {:?}\n", utype.rd());
}

fn print_r_type(instruction: Instruction, rtype: RType) {
    print!("Instruction = {:?}\n", instruction);
    print!("Register 1 = {:?}\n", rtype.rs1());
    print!("Register 2 = {:?}\n", rtype.rs2());
    print!("Destination register = {:?}\n", rtype.rd());
}

fn print_i_type(instruction: Instruction, itype: IType) {
	print!("Instruction = {:?}\n", instruction);
    print!("Register 1 = {:?}\n", itype.rs1());
    print!("Immediate = {:?}\n", itype.imm());
    print!("Destination register = {:?}\n", itype.rd());
}

fn print_s_type(instruction: Instruction, stype: SType) {
	print!("Instruction = {:?}\n", instruction);
    print!("Register 1 = {:?}\n", stype.rs1());
    print!("Register 1 = {:?}\n", stype.rs2());
    print!("Immediate = {:?}\n", stype.imm());
}

fn print_sh_type(instruction: Instruction, stype: ShiftType) {
	print!("Instruction = {:?}\n", instruction);
    print!("Register 1 = {:?}\n", stype.rs1());
    print!("Register 1 = {:?}\n", stype.rd());
    print!("shamt = {:?}\n", stype.shamt());
}

fn print(instruction: Instruction) {
    use Instruction::*;

    match instruction {
        Lui(itype) => print_u_type(instruction, itype),
        Auipc(itype) => print_u_type(instruction, itype),
        Jal(itype) => todo!(),
        Jalr(itype) => todo!(),
        Beq(itype) => todo!(),
        Bne(itype) => todo!(),
        Blt(itype) => todo!(),
        Bge(itype) => todo!(),
        Bltu(itype) => todo!(),
        Bgeu(itype) => todo!(),
        Lb(itype) => print_i_type(instruction, itype),
        Lh(itype) => print_i_type(instruction, itype),
        Lw(itype) => print_i_type(instruction, itype),
        Lbu(itype) => print_i_type(instruction, itype),
        Lhu(itype) => print_i_type(instruction, itype),
        Lwu(itype) => print_i_type(instruction, itype),
        Ld(itype) => print_i_type(instruction, itype),
        Sb(itype) => print_s_type(instruction, itype),
        Sh(itype) => print_s_type(instruction, itype),
        Sw(itype) => print_s_type(instruction, itype),
        Sd(itype) => print_s_type(instruction, itype),
        Addi(itype) => print_i_type(instruction, itype),
        Slti(itype) => print_i_type(instruction, itype),
        Sltiu(itype) => print_i_type(instruction, itype),
        Xori(itype) => print_i_type(instruction, itype),
        Ori(itype) => print_i_type(instruction, itype),
        Andi(itype) => print_i_type(instruction, itype),
        Slli(itype) => print_sh_type(instruction, itype),
        Srli(itype) => print_sh_type(instruction, itype),
        Srai(itype) => print_sh_type(instruction, itype),
        Add(itype) => print_r_type(instruction, itype),
        Sub(itype) => print_r_type(instruction, itype),
        Sll(itype) => print_r_type(instruction, itype),
        Slt(itype) => print_r_type(instruction, itype),
        Sltu(itype) => print_r_type(instruction, itype),
        Xor(itype) => print_r_type(instruction, itype),
        Srl(itype) => print_r_type(instruction, itype),
        Sra(itype) => print_r_type(instruction, itype),
        Or(itype) => print_r_type(instruction, itype),
        And(itype) => print_r_type(instruction, itype),
        Mul(itype) => print_r_type(instruction, itype),
        Mulh(itype) => print_r_type(instruction, itype),
        Mulhsu(itype) => print_r_type(instruction, itype),
        Mulhu(itype) => print_r_type(instruction, itype),
        Div(itype) => print_r_type(instruction, itype),
        Divu(itype) => print_r_type(instruction, itype),
        Rem(itype) => print_r_type(instruction, itype),
        Remu(itype) => print_r_type(instruction, itype),
        Fence(itype) => todo!(),
        FenceI => todo!(),
        Ecall => todo!(),
        Ebreak => todo!(),
        Uret => todo!(),
        Sret => todo!(),
        Mret => todo!(),
        Wfi => todo!(),
        SfenceVma(itype) => todo!(),
        Csrrw(itype) => todo!(),
        Csrrs(itype) => todo!(),
        Csrrc(itype) => todo!(),
        Csrrwi(itype) => todo!(),
        Csrrsi(itype) => todo!(),
        Csrrci(itype) => todo!(),
        Addiw(itype) => todo!(),
        Slliw(itype) => todo!(),
        Srliw(itype) => todo!(),
        Sraiw(itype) => todo!(),
        Addw(itype) => print_r_type(instruction, itype),
        Subw(itype) => print_r_type(instruction, itype),
        Sllw(itype) => print_r_type(instruction, itype),
        Srlw(itype) => print_r_type(instruction, itype),
        Sraw(itype) => print_r_type(instruction, itype),
        Mulw(itype) => print_r_type(instruction, itype),
        Divw(itype) => print_r_type(instruction, itype),
        Divuw(itype) => print_r_type(instruction, itype),
        Remw(itype) => print_r_type(instruction, itype),
        Remuw(itype) => print_r_type(instruction, itype),
        Illegal => todo!(),
        __Nonexhaustive => todo!(),
    };
}

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("riscv-cli-decoder")
        .version(crate_version!())
        .version("1.0")
        .author("Marnix Wieffering. <git@marnixwieffering.dev>")
        .about("Decodes RISCV instructions")
        .arg(
            Arg::with_name("decode")
                .short("d")
                .value_name("instruction")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let input = matches.value_of("decode").unwrap();
    let result = decode(input.parse::<u32>().unwrap()).unwrap();
	print(result);

    return Ok(());
}
