#[derive(Copy, Clone)]
pub enum RegId {
    A,
    B,
    C,
    D,
}

#[derive(Copy, Clone)]
pub enum Val {
    Imm(i64),
    Reg(RegId),
}

#[derive(Copy, Clone)]
pub enum Op {
    Cpy(Val, RegId),
    Inc(RegId),
    Dec(RegId),
    Jnz(Val, Val),
}

pub struct Computer {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    pc: i64,
    program: Vec<Op>,
}

impl Computer {
    pub fn new(program_str: &str) -> Computer {
        let program = parse_input(program_str);
        Computer {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            pc: 0,
            program,
        }
    }

    pub fn exec(&mut self) {
        while self.pc >= 0 && self.pc < self.program.len() as i64 {
            match self.program[self.pc as usize] {
                Op::Cpy(x, y) => {
                    self.set_reg(y, self.eval(&x));
                    self.pc += 1;
                }
                Op::Inc(x) => {
                    self.set_reg(x, self.get_reg(x) + 1);
                    self.pc += 1;
                }
                Op::Dec(x) => {
                    self.set_reg(x, self.get_reg(x) - 1);
                    self.pc += 1;
                }
                Op::Jnz(x, y) => {
                    if self.eval(&x) != 0 {
                        self.pc += self.eval(&y);
                    } else {
                        self.pc += 1;
                    }
                }
            }
        }
    }

    pub fn get_reg(&self, id: RegId) -> i64 {
        match id {
            RegId::A => self.a,
            RegId::B => self.b,
            RegId::C => self.c,
            RegId::D => self.d,
        }
    }

    pub fn set_reg(&mut self, id: RegId, v: i64) {
        match id {
            RegId::A => {
                self.a = v;
            }
            RegId::B => {
                self.b = v;
            }
            RegId::C => {
                self.c = v;
            }
            RegId::D => {
                self.d = v;
            }
        }
    }

    fn eval(&self, val: &Val) -> i64 {
        match val {
            Val::Imm(x) => *x,
            Val::Reg(r) => self.get_reg(*r),
        }
    }
}

fn to_regid(s: &str) -> Option<RegId> {
    let c = s.chars().next().unwrap();
    match c {
        'a' => Some(RegId::A),
        'b' => Some(RegId::B),
        'c' => Some(RegId::C),
        'd' => Some(RegId::D),
        _ => None,
    }
}

fn parse_val(s: &str) -> Val {
    match s.parse::<i64>() {
        Ok(x) => Val::Imm(x),
        Err(_) => Val::Reg(to_regid(s).unwrap()),
    }
}

fn parse_input(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| {
            let words: Vec<_> = line.split_whitespace().collect();
            match words[0] {
                "cpy" => Some(Op::Cpy(parse_val(words[1]), to_regid(words[2]).unwrap())),
                "inc" => Some(Op::Inc(to_regid(words[1]).unwrap())),
                "dec" => Some(Op::Dec(to_regid(words[1]).unwrap())),
                "jnz" => Some(Op::Jnz(parse_val(words[1]), parse_val(words[2]))),
                _ => None,
            }
        })
        .filter_map(|x| x)
        .collect()
}
