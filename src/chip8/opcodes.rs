#[derive(Debug)]
pub enum Opcode {
    OP00E0, // CLS
    OP00EE, // RET
    OP1NNN, // JP addr
    OP2NNN, // CALL addr
    OP3XKK, // SE Vx, byte
    OP4XKK, // SNE Vx, byte
    OP5XY0, // SE Vx, Vy
    OP6XKK, // LD Vx, byte
    OP7XKK, // ADD Vx, byte
    OP8XY0, // LD Vx, Vy
    OP8XY1, // OR Vx, Vy
    OP8XY2, // AND Vx, Vy
    OP8XY3, // XOR Vx, Vy
    OP8XY4, // ADD Vx, Vy
    OP8XY5, // SUB Vx, Vy
    OP8XY6, // SHR Vx {, Vy}
    OP8XY7, // SUBN Vx, Vy
    OP8XYE, // SHL Vx {, Vy}
    OP9XY0, // SNE Vx, Vy
    OPANNN, // LD I, addr
    OPBNNN, // JP V0, addr
    OPCXKK, // RND Vx, byte
    OPDXYN, // DRW Vx, Vy, nibble
    OPEX9E, // SKP Vx
    OPEXA1, // SKNP Vx
    OPFX07, // LD Vx, DT
    OPFX0A, // LD Vx, K
    OPFX15, // LD DT, Vx
    OPFX18, // LD ST, Vx
    OPFX1E, // ADD I, Vx
    OPFX29, // LD F, Vx
    OPFX33, // LD B, Vx
    OPFX55, // LD [I], Vx
    OPFX65, // LD Vx, [I]
}
