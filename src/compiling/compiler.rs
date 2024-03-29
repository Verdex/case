
use crate::data::*;

pub fn compile(input : Vec<DefOrExpr>) -> Result<Vec<Il>, CompileError>  {
    
    let output = input.into_iter().map(|x| match x {
        DefOrExpr::Expr(e) => compile_expr(e),
        DefOrExpr::FnDef(fd) => compile_fn_def(fd),
        _ => panic!("!"),
    }).collect::<Result<Vec<_>, _>>()?;

    Ok(output.into_iter().flatten().collect())

}

fn compile_fn_def(input : FnDef) -> Result<Vec<Il>, CompileError> {
    // TODO need to handle parameters
    let expr = compile_expr(input.body)?;
    Ok(vec![ Il::Push(IlData::Code(expr))
           , Il::Push(IlData::String(input.name)) 
           , Il::Def
           ])
}

fn compile_expr(input : Expr) -> Result<Vec<Il>, CompileError> {
    match input { 
        Expr::Var { value, .. } => todo!(), 
        Expr::Float { value, .. } => Ok(vec![Il::Push(IlData::Float(value))]),
        Expr::Symbol { value, .. } => Ok(vec![Il::Push(IlData::Symbol(value))]),
        Expr::Call { .. } => todo!(),
        Expr::TupleCons { params, .. } => {
            let param_count = params.len();
            let mut params = params.into_iter()
                                   .map(compile_expr)
                                   .collect::<Result<Vec<Vec<Il>>, _>>()?
                                   .into_iter()
                                   .flatten()
                                   .collect::<Vec<Il>>();
            params.push(Il::TupleCons(param_count));
            Ok(params)
        },
    }
}
