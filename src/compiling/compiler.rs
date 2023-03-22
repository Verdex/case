
use crate::data::*;

pub fn compile(input : Vec<DefOrExpr>) -> Result<Vec<Il>, CompileError>  {

    Ok(vec![Il::Print])
}

fn compile_expr(input : Expr) -> Result<Vec<Il>, CompileError> {
    match input { 
        Expr::Float { value, .. } => Ok(vec![Il::Push(IlData::Float(value))]),
        Expr::Symbol { value, .. } => Ok(vec![Il::Push(IlData::Symbol(value))]),
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
