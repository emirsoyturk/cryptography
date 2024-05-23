pub struct R1CS {
    pub program: Program,
    pub solution: Vec<i32>,
    pub witness: Vec<i32>,
}

pub struct Variable {
    pub name: String,
    pub value: i32,
}

pub struct Constraint {
    pub left: Variable,
    pub right: Variable,
    pub operator: String,
}

pub struct Program {
    pub variables: Vec<Variable>,
    pub constraints: Vec<Constraint>,
}

impl Program {
    pub fn new(program: String) -> Program {
        let mut p = Program {
            variables: vec![],
            constraints: vec![],
        };

        p.parse(program);

        p
    }

    fn add_variable(&mut self, name: String) {
        let variable = Variable { name, value: 0 };

        if !self.variables.contains(&variable) {
            self.variables.push(variable);
        }
    }

    fn add_constraint(&mut self, left: Variable, right: Variable, operator: String) {
        let constraint = Constraint {
            left,
            right,
            operator,
        };

        self.constraints.push(constraint);
    }

    fn parse(&mut self, program: String) {
        let lines = program.split(";");
        for line in lines {
            let parts: Vec<&str> = line.split("=").collect();
            if parts.len() != 2 {
                continue;
            }

            let output = parts[0].trim();
            let expression = parts[1].trim();
            let parts: Vec<&str> = expression.split(" ").collect();
            let left = parts[0].trim();
            let operator = parts[1].trim();
            let right = parts[2].trim();

            let var_left = Variable {
                name: left.to_string(),
                value: 0,
            };

            let var_right = Variable {
                name: right.to_string(),
                value: 0,
            };

            self.add_constraint(var_left, var_right, operator.to_string());
            self.add_variable(left.to_string());
            self.add_variable(right.to_string());
            self.add_variable(output.to_string());
        }
    }
}

impl R1CS {
    pub fn new(program: String) -> Self {
        R1CS {
            program: Program::new(program),
            solution: vec![],
            witness: vec![],
        }
    }

    pub fn prove(&mut self, variables: Vec<i32>) {
        if variables.len() != self.program.variables.len() {
            panic!("Invalid number of variables");
        }
        self.witness.push(1);
        for (i, _) in self.program.variables.iter().enumerate() {
            self.witness.push(variables[i]);
        }

        println!("Witness: {:?}", self.witness);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        let program = Program::new("v = x * x; z = v + y;".to_string());
        assert_eq!(program.variables.len(), 2);
        assert_eq!(program.constraints.len(), 2);
    }

    #[test]
    fn test_r1cs() {
        let mut r1cs = R1CS::new("v = x * x; z = v + y;".to_string());
        r1cs.prove(vec![2, 3]);
    }
}
