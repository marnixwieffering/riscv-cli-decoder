use riscv_decode::{
	types::{IType, RType, SType, ShiftType, UType},
	Instruction,
};

fn print_u_type(instruction: Instruction, u_type: UType) {
	print!(
		"Instruction = {:?}\n Immediate = {:?}\n Destination register = {:?}\n",
		instruction,
		u_type.imm(),
		u_type.rd()
	);
}

fn print_r_type(instruction: Instruction, r_type: RType) {
	print!(
		"Instruction = {:?}\nRegister 1 = {:?}\nRegister 2 = {:?}\nDestination register = {:?}\n",
		instruction,
		r_type.rs1(),
		r_type.rs2(),
		r_type.rd()
	);
}

fn print_i_type(instruction: Instruction, i_type: IType) {
	print!(
		"Instruction = {:?}\nRegister 1 = {:?}\nImmediate = {:?}\nDestination register = {:?}\n",
		instruction,
		i_type.rs1(),
		i_type.imm(),
		i_type.rd()
	);
}

fn print_s_type(instruction: Instruction, s_type: SType) {
	print!(
		"Instruction = {:?}\nRegister 1 = {:?}\nRegister 2 = {:?}\nImmediate = {:?}\n",
		instruction,
		s_type.rs1(),
		s_type.rs2(),
		s_type.imm()
	);
}

fn print_sh_type(instruction: Instruction, s_type: ShiftType) {
	print!(
		"Instruction = {:?}\nRegister 1 = {:?}\nDestination register = {:?}\nshamt = {:?}\n",
		instruction,
		s_type.rs1(),
		s_type.rd(),
		s_type.shamt()
	);
}

pub fn print(instruction: Instruction) {
	use Instruction::*;

	match instruction {
		Lui(r#type) => print_u_type(instruction, r#type),
		Auipc(r#type) => print_u_type(instruction, r#type),
		Jal(r#_type) => todo!(),
		Jalr(r#_type) => todo!(),
		Beq(r#_type) => todo!(),
		Bne(r#_type) => todo!(),
		Blt(r#_type) => todo!(),
		Bge(r#_type) => todo!(),
		Bltu(r#_type) => todo!(),
		Bgeu(r#_type) => todo!(),
		Lb(r#type) => print_i_type(instruction, r#type),
		Lh(r#type) => print_i_type(instruction, r#type),
		Lw(r#type) => print_i_type(instruction, r#type),
		Lbu(r#type) => print_i_type(instruction, r#type),
		Lhu(r#type) => print_i_type(instruction, r#type),
		Lwu(r#type) => print_i_type(instruction, r#type),
		Ld(r#type) => print_i_type(instruction, r#type),
		Sb(r#type) => print_s_type(instruction, r#type),
		Sh(r#type) => print_s_type(instruction, r#type),
		Sw(r#type) => print_s_type(instruction, r#type),
		Sd(r#type) => print_s_type(instruction, r#type),
		Addi(r#type) => print_i_type(instruction, r#type),
		Slti(r#type) => print_i_type(instruction, r#type),
		Sltiu(r#type) => print_i_type(instruction, r#type),
		Xori(r#type) => print_i_type(instruction, r#type),
		Ori(r#type) => print_i_type(instruction, r#type),
		Andi(r#type) => print_i_type(instruction, r#type),
		Slli(r#type) => print_sh_type(instruction, r#type),
		Srli(r#type) => print_sh_type(instruction, r#type),
		Srai(r#type) => print_sh_type(instruction, r#type),
		Add(r#type) => print_r_type(instruction, r#type),
		Sub(r#type) => print_r_type(instruction, r#type),
		Sll(r#type) => print_r_type(instruction, r#type),
		Slt(r#type) => print_r_type(instruction, r#type),
		Sltu(r#type) => print_r_type(instruction, r#type),
		Xor(r#type) => print_r_type(instruction, r#type),
		Srl(r#type) => print_r_type(instruction, r#type),
		Sra(r#type) => print_r_type(instruction, r#type),
		Or(r#type) => print_r_type(instruction, r#type),
		And(r#type) => print_r_type(instruction, r#type),
		Mul(r#type) => print_r_type(instruction, r#type),
		Mulh(r#type) => print_r_type(instruction, r#type),
		Mulhsu(r#type) => print_r_type(instruction, r#type),
		Mulhu(r#type) => print_r_type(instruction, r#type),
		Div(r#type) => print_r_type(instruction, r#type),
		Divu(r#type) => print_r_type(instruction, r#type),
		Rem(r#type) => print_r_type(instruction, r#type),
		Remu(r#type) => print_r_type(instruction, r#type),
		Fence(r#_type) => todo!(),
		FenceI => todo!(),
		Ecall => todo!(),
		Ebreak => todo!(),
		Uret => todo!(),
		Sret => todo!(),
		Mret => todo!(),
		Wfi => todo!(),
		SfenceVma(r#_type) => todo!(),
		Csrrw(r#_type) => todo!(),
		Csrrs(r#_type) => todo!(),
		Csrrc(r#_type) => todo!(),
		Csrrwi(r#_type) => todo!(),
		Csrrsi(r#_type) => todo!(),
		Csrrci(r#_type) => todo!(),
		Addiw(r#_type) => todo!(),
		Slliw(r#_type) => todo!(),
		Srliw(r#_type) => todo!(),
		Sraiw(r#_type) => todo!(),
		Addw(r#type) => print_r_type(instruction, r#type),
		Subw(r#type) => print_r_type(instruction, r#type),
		Sllw(r#type) => print_r_type(instruction, r#type),
		Srlw(r#type) => print_r_type(instruction, r#type),
		Sraw(r#type) => print_r_type(instruction, r#type),
		Mulw(r#type) => print_r_type(instruction, r#type),
		Divw(r#type) => print_r_type(instruction, r#type),
		Divuw(r#type) => print_r_type(instruction, r#type),
		Remw(r#type) => print_r_type(instruction, r#type),
		Remuw(r#type) => print_r_type(instruction, r#type),
		Illegal => todo!(),
		__Nonexhaustive => todo!(),
	};
}
