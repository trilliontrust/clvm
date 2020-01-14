use super::eval::{EvalErr, Reduction};
use super::number::Number;
use super::serialize::{node_from_stream, node_to_stream};
use super::sexp::Node;
use sha2::{Digest, Sha256};
use std::io::Cursor;
use std::io::{Seek, SeekFrom, Write};

pub fn op_wrap(args: &Node) -> Result<Reduction, EvalErr> {
    let mut buffer = Cursor::new(Vec::new());
    if node_to_stream(&args.first()?, &mut buffer).is_ok() {
        let vec = buffer.into_inner();
        return Ok(Node::blob_u8(&vec).into());
    }
    panic!("op_wrap panic")
}

pub fn op_unwrap(args: &Node) -> Result<Reduction, EvalErr> {
    let mut buffer = Cursor::new(Vec::new());
    if let Some(b) = args.first()?.as_blob() {
        if buffer.write_all(&b).is_ok() && buffer.seek(SeekFrom::Start(0)).is_ok() {
            if let Ok(node) = node_from_stream(&mut buffer) {
                return Ok(node.into());
            }
        }
    }
    args.err("bad stream")
}

pub fn op_sha256(args: &Node) -> Result<Reduction, EvalErr> {
    let mut hasher = Sha256::new();
    for arg in args.clone() {
        match arg.as_blob() {
            Some(blob) => hasher.input(blob),
            None => return args.err("atom expected"),
        }
    }
    Ok(Node::blob_u8(&hasher.result()).into())
}

pub fn op_sha256_tree(args: &Node) -> Result<Reduction, EvalErr> {
    let mut hasher = Sha256::new();
    node_to_stream(&args.first()?, &mut hasher)?;
    Ok(Node::blob_u8(&hasher.result()).into())
}

pub fn op_add(args: &Node) -> Result<Reduction, EvalErr> {
    let mut total: Number = 0.into();
    for arg in args.clone() {
        let v: Option<Number> = Option::from(&arg);
        match v {
            Some(value) => total += value,
            None => return args.err("+ takes integer arguments"),
        }
    }
    let total: Node = total.into();
    Ok(total.into())
}

pub fn op_subtract(args: &Node) -> Result<Reduction, EvalErr> {
    let mut total: Number = 0.into();
    let mut is_first = true;
    for arg in args.clone() {
        let v: Option<Number> = Option::from(&arg);
        match v {
            Some(value) => {
                if is_first {
                    total += value;
                } else {
                    total -= value;
                };
                is_first = false;
            }
            None => return args.err("- takes integer arguments"),
        }
    }
    let total: Node = total.into();
    Ok(total.into())
}

pub fn op_multiply(args: &Node) -> Result<Reduction, EvalErr> {
    let mut total: Number = 1.into();
    for arg in args.clone() {
        let v: Option<Number> = Option::from(&arg);
        match v {
            Some(value) => total *= value,
            None => return args.err("* takes integer arguments"),
        }
    }
    let total: Node = total.into();
    Ok(total.into())
}

pub fn op_gr(args: &Node) -> Result<Reduction, EvalErr> {
    let a0 = args.first()?;
    let v0: Option<Number> = Option::from(&a0);
    let a1 = args.rest()?.first()?;
    let v1: Option<Number> = Option::from(&a1);
    if let Some(n0) = v0 {
        if let Some(n1) = v1 {
            return Ok(if n0 > n1 {
                Node::blob_u8(&[1]).into()
            } else {
                Node::null().into()
            });
        }
    }
    args.err("> on list")
}
