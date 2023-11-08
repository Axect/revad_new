use crate::core::{Graph, Expr};

pub fn gradient<F: Fn(&[Expr]) -> Expr>(f: F, x: &[f64]) -> Vec<f64> {
    let mut graph = Graph::default();
    let var_vec = x.iter().map(|x| graph.var(*x)).collect::<Vec<_>>();
    let expr_vec = var_vec.iter().map(|x| Expr::Symbol(*x)).collect::<Vec<_>>();
    let result_expr = f(&expr_vec);

    graph.compile(result_expr);
    let _ = graph.forward();
    graph.backward();

    var_vec.iter().map(|x| graph.get_gradient(*x)).collect::<Vec<_>>()
}

/// graph is already compiled
pub fn gradient_cached(g: &mut Graph, x: &[f64]) -> (f64, Vec<f64>) {
    g.reset();
    g.subs_vars(x);
    let result = g.forward();
    g.backward();
    let grads = g.get_gradients();

    (result, grads)
}
