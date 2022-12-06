pub mod structure;

use crate::parser::file::*;

// 1: T[x] = x
// 2: T[(a b)] = (T[a] T[b])
// 3: T[\x.E] = (K T[E])   if x is not in E
// 4: T[\x.x] = I
// 5: T[\x.\y.E] = T[\x.T[\y.E]]   if x is in E
// 6: T[\x.(a b)] = (S T[\x.a] T[\x.b])   if x is in a or b
pub fn convert(lambda: &Term, offset: u32) -> Term {
    if let Term::Var(v) = lambda {
        return Term::Var(Variable {
            index: v.index - offset,
        });
    } else if let Term::Lam(l) = &lambda {
        if occurs_free(&*l.body, 0) {
            return match &*l.body {
                Term::App(a) => free_app(a.clone(), offset),
                // If the body of a lambda was just a variable, and the lambda's
                // argument is in the body, then it is \x.x
                Term::Var(_) => Term::Com(structure::Combinator::I),
                Term::Lam(_) => convert(
                    &Term::Lam(Lambda {
                        body: Box::new(convert(&*l.body, offset + 1)),
                    }),
                    offset,
                ),
                Term::Com(_) => panic!("unreachable"),
            };
        } else {
            return Term::App(Application {
                function: Box::new(Term::Com(structure::Combinator::K)),
                argument: Box::new(convert(&*l.body, offset)),
            });
        }
    } else if let Term::App(a) = lambda {
        return Term::App(Application {
            function: Box::new(convert(&*a.function, offset)),
            argument: Box::new(convert(&*a.argument, offset)),
        });
    } else {
        // If it's a combinator we don't need to do anything
        return lambda.clone();
    }
}

pub fn free_app(app: Application, offset: u32) -> Term {
    Term::App(Application {
        function: Box::new(Term::App(Application {
            function: Box::new(Term::Com(structure::Combinator::S)),
            argument: Box::new(convert(&Term::Lam(Lambda { body: app.function }), offset)),
        })),
        argument: Box::new(convert(&Term::Lam(Lambda { body: app.argument }), offset)),
    })
}

pub fn occurs_free(lambda: &Term, offset: u32) -> bool {
    match lambda {
        Term::Var(v) => v.index == offset,
        Term::App(a) => occurs_free(&*a.function, offset) || occurs_free(&*a.argument, offset),
        Term::Lam(l) => occurs_free(&*l.body, offset + 1),
        Term::Com(_) => false,
    }
}
