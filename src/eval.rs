use super::sexp::Node;

#[derive(Debug, Clone)]
pub struct EvalErr(pub Node, pub String);

pub struct Reduction(pub Node, pub u32);

pub type FApply = fn(&Node) -> Result<Reduction, EvalErr>;

pub type FApply2 = Box<dyn FnMut(&Node, &Node) -> Result<Reduction, EvalErr>>;

pub type FLookup = [Option<FApply>; 256];

impl From<std::io::Error> for EvalErr {
    fn from(err: std::io::Error) -> Self {
        EvalErr(Node::blob("std::io::Error"), err.to_string())
    }
}

impl From<Node> for Reduction {
    fn from(node: Node) -> Self {
        Reduction(node, 1)
    }
}

impl Node {
    pub fn err(&self, msg: &str) -> Result<Reduction, EvalErr> {
        Err(EvalErr(self.clone(), msg.into()))
    }

    pub fn node_err(&self, msg: &str) -> Result<Node, EvalErr> {
        Err(EvalErr(self.clone(), msg.into()))
    }
}

fn eval_params(
    params: &Node,
    env: &Node,
    current_cost: u32,
    max_cost: u32,
    f_table: &FLookup,
    mut apply2: &mut FApply2,
    op_quote: u8,
    op_args: u8,
) -> Result<Reduction, EvalErr> {
    let iter = params.clone();
    let mut new_cost = current_cost;
    let mut vec: Vec<Node> = Vec::new();
    for item in iter {
        let r = eval(
            &item,
            &env,
            new_cost,
            max_cost,
            f_table,
            &mut apply2,
            op_quote,
            op_args,
        )?;
        vec.push(r.0);
        new_cost += r.1;
        if new_cost >= max_cost {
            return item.err("exceed max cost");
        }
    }
    Ok(Reduction(Node::from_list(vec), new_cost))
}

fn apply(
    operator: &Node,
    params: &Node,
    f_table: &FLookup,
    apply2: &mut FApply2,
) -> Result<Reduction, EvalErr> {
    let op_8: Option<u8> = operator.clone().into();
    if let Some(op_8) = op_8 {
        if let Some(f) = f_table[op_8 as usize] {
            return f(&params);
        }
    };
    apply2(operator, params)
}

pub fn eval(
    form: &Node,
    env: &Node,
    current_cost: u32,
    max_cost: u32,
    f_table: &FLookup,
    apply2: &mut FApply2,
    op_quote: u8,
    op_args: u8,
) -> Result<Reduction, EvalErr> {
    match form.as_pair() {
        None => form.err("not a list"),
        Some((left, right)) => {
            if left.is_pair() {
                let r = eval(
                    &left,
                    &env,
                    current_cost,
                    max_cost,
                    f_table,
                    apply2,
                    op_quote,
                    op_args,
                )?;
                match r {
                    Reduction(result, new_cost) => eval(
                        &result.first()?,
                        &result.rest()?,
                        new_cost,
                        max_cost,
                        f_table,
                        apply2,
                        op_quote,
                        op_args,
                    ),
                }
            } else {
                let as_operator: Option<u8> = left.clone().into();
                if let Some(opcode) = as_operator {
                    if opcode == op_quote {
                        return {
                            let rest = form.rest()?;
                            if rest.nullp() || !rest.rest()?.nullp() {
                                form.err("quote requires exactly 1 parameter")
                            } else {
                                Ok(Reduction(right.first()?, current_cost + 1))
                            }
                        };
                    } else if opcode == op_args {
                        return { Ok(Reduction(env.clone(), current_cost + 1)) };
                    }
                }
                let Reduction(params, new_cost) = eval_params(
                    &right,
                    env,
                    current_cost,
                    max_cost,
                    f_table,
                    apply2,
                    op_quote,
                    op_args,
                )?;

                let Reduction(r, apply_cost) = apply(&left, &params, f_table, apply2)?;
                Ok(Reduction(r, apply_cost + new_cost))
            }
        }
    }
}
