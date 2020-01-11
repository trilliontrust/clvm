use super::sexp::Node;

#[derive(Debug, Clone)]
pub struct EvalErr(pub Node, pub String);

pub struct Reduction(pub Node, pub u32);

pub type FApply = fn(&Node) -> Result<Reduction, EvalErr>;

pub type FApply2 = Box<dyn FnMut(&Node, &Node) -> Result<Reduction, EvalErr>>;

pub type FEval = fn(
    form: &Node,
    env: &Node,
    current_cost: u32,
    max_cost: u32,
    f_table: &FLookup,
    apply2: &mut FApply2,
) -> Result<Reduction, EvalErr>;

pub struct FEval1(fn(feval: &FEval1, form: &Node) -> ());

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

const OP_QUOTE: u8 = 1;
const OP_ARGS: u8 = 3;

fn as_operator(item: &Node) -> Option<u8> {
    let blob = item.as_blob()?;
    let len = blob.len();
    if len == 0 {
        Some(0)
    } else if len == 1 {
        Some(blob[0])
    } else {
        None
    }
}

fn eval_params(
    params: &Node,
    env: &Node,
    current_cost: u32,
    max_cost: u32,
    f_table: &FLookup,
    mut apply2: &mut FApply2,
) -> Result<Reduction, EvalErr> {
    let iter = params.clone();
    let mut new_cost = current_cost;
    let mut vec: Vec<Node> = Vec::new();
    for item in iter {
        let r = eval(&item, &env, new_cost, max_cost, f_table, &mut apply2)?;
        vec.push(r.0);
        new_cost += r.1;
        if new_cost >= max_cost {
            return item.err("exceed max cost");
        }
    }
    Ok(Reduction(Node::from_list(vec), new_cost))
}

fn apply(
    operator: u8,
    rest: &Node,
    env: &Node,
    current_cost: u32,
    max_cost: u32,
    f_table: &FLookup,
    apply2: &mut FApply2,
) -> Result<Reduction, EvalErr> {
    let Reduction(params, new_cost) =
        eval_params(&rest, env, current_cost, max_cost, f_table, apply2)?;

    match f_table[operator as usize] {
        // TODO: call apply2 here
        None => Node::blob_u8(&[operator]).err("unknown operand"),
        Some(f) => match f(&params) {
            Ok(Reduction(node, cost)) => Ok(Reduction(node, cost + new_cost)),
            Err(e) => Err(e),
        },
    }
}

pub fn eval(
    form: &Node,
    env: &Node,
    current_cost: u32,
    max_cost: u32,
    f_table: &FLookup,
    apply2: &mut FApply2,
) -> Result<Reduction, EvalErr> {
    match form.as_pair() {
        None => form.err("not a list"),
        Some((left, right)) => {
            if let Some(_) = left.as_pair() {
                let r = eval(&left, &env, current_cost, max_cost, f_table, apply2)?;
                match r {
                    Reduction(result, new_cost) => eval(
                        &result.first()?,
                        &result.rest()?,
                        new_cost,
                        max_cost,
                        f_table,
                        apply2,
                    ),
                }
            } else {
                match as_operator(&left) {
                    Some(OP_QUOTE) => {
                        let rest = form.rest()?;
                        if rest.nullp() || !rest.rest()?.nullp() {
                            form.err("quote requires exactly 1 parameter")
                        } else {
                            Ok(Reduction(right.first()?.clone(), current_cost + 1))
                        }
                    }
                    Some(OP_ARGS) => Ok(Reduction(env.clone(), current_cost + 1)),
                    Some(operator) => apply(
                        operator,
                        &right,
                        env,
                        current_cost,
                        max_cost,
                        f_table,
                        apply2,
                    ),
                    // TODO: handle complex operator like "com"
                    _ => {
                        let Reduction(params, new_cost) =
                            eval_params(&right, env, current_cost, max_cost, f_table, apply2)?;
                        let partial_reduction = apply2(&left, &params);
                        match partial_reduction {
                            Ok(Reduction(r, partial_cost)) => {
                                Ok(Reduction(r, new_cost + partial_cost))
                            }
                            Err(err) => Err(err),
                        }
                    }
                }
            }
        }
    }
}
