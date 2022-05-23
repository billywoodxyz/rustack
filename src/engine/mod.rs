use std::collections::HashMap;

pub enum Instruction {
    LoadVal { val: u16 },
    WriteVar { name: String },
    ReadVar { name: String },
    Add,
    Multiply,
    CmpNe,
    ConditionLoop { offset: u16 },
    ReturnValue,
}

#[derive(Default)]
pub struct ByteCodeEngine {
    ip: u16,
    sp: u16,
    sf: HashMap<String, u16>,
    stack: Vec<u16>,
    pub ixs: Vec<Instruction>,
}

impl ByteCodeEngine {
    pub fn run(&mut self) -> Option<u16> {
        let mut return_value = None;
        if self.ip == 0 {
            for ix in self.ixs.iter() {
                if let Instruction::WriteVar { name } = ix {
                    if !self.sf.contains_key(name) {
                        self.stack.push(0);
                        self.sf.insert(name.to_string(), self.sp);
                        self.sp += 1;
                    }
                }
            }
        }
        let ix = &self.ixs[self.ip as usize];
        match ix {
            Instruction::LoadVal { val } => {
                self.sp += 1;
                self.stack.push(*val);
            }
            Instruction::ReadVar { name } => {
                self.sp += 1;
                let addr = *self.sf.get_key_value(name).unwrap().1 as usize;
                self.stack.push(self.stack[addr]);
            }
            Instruction::WriteVar { name } => {
                let addr = *self.sf.get_key_value(name).unwrap().1 as usize;
                self.stack[addr] = self.stack.pop().unwrap();
                self.sp -= 1;
            }
            Instruction::Add => {
                let left = self.stack.pop().unwrap();
                self.sp -= 1;
                let right = self.stack.pop().unwrap();
                self.sp -= 1;
                self.sp += 1;
                self.stack.push(left + right);
            }
            Instruction::Multiply => {
                let left = self.stack.pop().unwrap();
                self.sp -= 1;
                let right = self.stack.pop().unwrap();
                self.sp -= 1;
                self.sp += 1;
                self.stack.push(left * right);
            }
            Instruction::CmpNe => {
                let left = self.stack.pop().unwrap();
                self.sp -= 1;
                let right = self.stack.pop().unwrap();
                self.sp -= 1;
                self.sp += 1;
                let val = if left != right { 1 } else { 0 };
                self.stack.push(val);
            }
            Instruction::ConditionLoop { offset } => {
                let condition = self.stack.pop().unwrap();
                self.sp -= 1;
                if condition == 1 {
                    self.ip -= offset;
                }
            }
            Instruction::ReturnValue => {
                return_value = Some(self.stack.pop().unwrap());
                self.sp -= 1;
            }
        }
        self.ip += 1;
        if self.ip == self.ixs.len() as u16 {
            self.ip = 0;
            self.sp = 0;
            self.sf = HashMap::default();
            self.stack = vec![];
        }
        return return_value;
    }
}

/// Design proposals
/// SEND_CHANNEL, RECV_CHANNEL and SPAWN
/// Introduce a channel_frame(similar to sf) with a HashMap of name & stack address to channel map.
/// For blocking feature, we can introduce infinite loop that checks stack address of channel.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let ixs: Vec<Instruction> = vec![
            Instruction::LoadVal { val: 1 },
            Instruction::WriteVar {
                name: "x".to_string(),
            },
            Instruction::LoadVal { val: 2 },
            Instruction::WriteVar {
                name: "y".to_string(),
            },
            Instruction::ReadVar {
                name: "x".to_string(),
            },
            Instruction::LoadVal { val: 1 },
            Instruction::Add,
            Instruction::ReadVar {
                name: "y".to_string(),
            },
            Instruction::Multiply,
            Instruction::ReturnValue,
        ];
        let mut engine = ByteCodeEngine::default();
        engine.ixs = ixs;
        for idx in 0..engine.ixs.len() {
            let result = engine.run();
            if idx == engine.ixs.len() - 1 {
                assert_eq!(result.unwrap(), 4);
            }
        }
    }

    #[test]
    fn test_loop() {
        let ixs: Vec<Instruction> = vec![
            Instruction::LoadVal { val: 1 },
            Instruction::WriteVar {
                name: "x".to_string(),
            },
            Instruction::ReadVar {
                name: "x".to_string(),
            },
            Instruction::LoadVal { val: 1 },
            Instruction::Add,
            Instruction::WriteVar {
                name: "x".to_string(),
            },
            Instruction::ReadVar {
                name: "x".to_string(),
            },
            Instruction::LoadVal { val: 5 },
            Instruction::CmpNe,
            Instruction::ConditionLoop { offset: 8 },
            Instruction::ReadVar {
                name: "x".to_string(),
            },
            Instruction::ReturnValue,
        ];
        let mut engine = ByteCodeEngine::default();
        engine.ixs = ixs;
        let mut total_runs = 0;
        loop {
            total_runs += 1;
            if let Some(return_value) = engine.run() {
                assert_eq!(return_value, 5);
                assert_eq!(total_runs, 36);
                break;
            }
        }
    }
}
