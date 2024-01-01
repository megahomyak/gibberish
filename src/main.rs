use std::rc::Rc;

pub struct Context {
    before: String,
    after: String,
}

pub struct Executor {
    pub names: preftree::PrefixTree<char, Rc<dyn Fn(Context, &mut Executor) -> Context>>,
}

impl Executor {
    pub fn execute(&mut self, program: String) -> String {
        let mut context = Context {
            before: "".into(),
            after: program,
        };
        loop {
            let mut after_chars = context.after.chars();
            match self.names.get_exact_match_mut(&mut after_chars) {
                Some(function) => {
                    let function = function.clone();
                    context.after = after_chars.as_str().to_owned();
                    context = function(context, self);
                }
                None => {
                    return context.before + &context.after;
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
