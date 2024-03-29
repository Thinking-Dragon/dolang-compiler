#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<Box<ASTNode>>),
    Data(String, Vec<Box<ASTNode>>),
    Group(String, Vec<Box<ASTNode>>, Vec<Box<ASTNode>>),
    Do(String, Vec<Box<ASTNode>>),
    Run(Vec<String>, Vec<Box<ASTNode>>),
    Parameter(String, String),
    Field(String, String),
    FieldValue(String, String),
    Value(String),
    Expression(Box<ASTNode>),
    Declaration(String, Box<ASTNode>),
    DataInstanciation(String, Vec<Box<ASTNode>>),
    CreateInstruction(String, Vec<Box<ASTNode>>),
    Sum(Box<ASTNode>, Box<ASTNode>),
    Substraction(Box<ASTNode>, Box<ASTNode>),
    Multiplication(Box<ASTNode>, Box<ASTNode>),
    Division(Box<ASTNode>, Box<ASTNode>),
    Modulo(Box<ASTNode>, Box<ASTNode>),
    If(Box<ASTNode>, Vec<Box<ASTNode>>),
    Foreach(Vec<String>, Vec<Box<ASTNode>>, Vec<Box<ASTNode>>),
    For(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>, Vec<Box<ASTNode>>),
}

impl ASTNode {
    pub fn new_data(name: String, fields: Vec<ASTNode>) -> ASTNode {
        ASTNode::Data(name, fields.into_iter().map(Box::new).collect())
    }

    pub fn new_group(name: String, parameters: Vec<ASTNode>, data_instanciations: Vec<ASTNode>) -> ASTNode {
        ASTNode::Group(
            name,
            parameters.into_iter().map(Box::new).collect(),
            data_instanciations.into_iter().map(Box::new).collect()
        )
    }

    pub fn new_do(name: String, instructions: Vec<ASTNode>) -> ASTNode {
        ASTNode::Do(name, instructions.into_iter().map(Box::new).collect())
    }

    pub fn new_run(actions_to_do: Vec<String>, instructions: Vec<ASTNode>) -> ASTNode {
        ASTNode::Run(actions_to_do, instructions.into_iter().map(Box::new).collect())
    }

    pub fn new_create_instruction(groupe_name: String, parameter_values: Vec<ASTNode>) -> ASTNode {
        ASTNode::CreateInstruction(groupe_name, parameter_values.into_iter().map(Box::new).collect())
    }

    pub fn new_sum(lhs: ASTNode, rhs: ASTNode) -> ASTNode {
        ASTNode::Sum(Box::new(lhs), Box::new(rhs))
    }

    pub fn new_substraction(lhs: ASTNode, rhs: ASTNode) -> ASTNode {
        ASTNode::Substraction(Box::new(lhs), Box::new(rhs))
    }

    pub fn new_multiplication(lhs: ASTNode, rhs: ASTNode) -> ASTNode {
        ASTNode::Multiplication(Box::new(lhs), Box::new(rhs))
    }

    pub fn new_division(lhs: ASTNode, rhs: ASTNode) -> ASTNode {
        ASTNode::Division(Box::new(lhs), Box::new(rhs))
    }

    pub fn new_modulo(lhs: ASTNode, rhs: ASTNode) -> ASTNode {
        ASTNode::Modulo(Box::new(lhs), Box::new(rhs))
    }

    pub fn new_parameter(name: String, parameter_type: String) -> ASTNode {
        ASTNode::Parameter(name, parameter_type)
    }

    pub fn new_field(name: String, field_type: String) -> ASTNode {
        ASTNode::Field(name, field_type)
    }

    pub fn new_field_value(name: String, value: String) -> ASTNode {
        ASTNode::FieldValue(name, value)
    }

    pub fn new_value(value: String) -> ASTNode {
        ASTNode::Value(value)
    }

    pub fn new_expression(value: ASTNode) -> ASTNode {
        ASTNode::Expression(Box::new(value))
    }

    pub fn new_declaration(variable_name: String, value: ASTNode) -> ASTNode {
        ASTNode::Declaration(variable_name, Box::new(value))
    }

    pub fn new_data_instanciation(data_name: String, field_values: Vec<ASTNode>) -> ASTNode {
        ASTNode::DataInstanciation(data_name, field_values.into_iter().map(Box::new).collect())
    }

    pub fn new_if(condition: ASTNode, instructions: Vec<ASTNode>) -> ASTNode {
        ASTNode::If(Box::new(condition), instructions.into_iter().map(Box::new).collect())
    }

    pub fn new_foreach(values: Vec<String>, collections: Vec<ASTNode>, instructions: Vec<ASTNode>) -> ASTNode {
        ASTNode::Foreach(
            values.clone(),
            collections.into_iter().map(Box::new).collect(),
            instructions.into_iter().map(Box::new).collect()
        )
    }

    pub fn new_for(declaration: ASTNode, condition: ASTNode, progression: ASTNode, instructions: Vec<ASTNode>) -> ASTNode {
        ASTNode::For(
            Box::new(declaration),
            Box::new(condition),
            Box::new(progression),
            instructions.into_iter().map(Box::new).collect()
        )
    }

    pub fn new_program(statements: Vec<ASTNode>) -> ASTNode {
        ASTNode::Program(statements.into_iter().map(Box::new).collect())
    }
}
