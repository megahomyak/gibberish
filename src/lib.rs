use preftree::PrefixTree;
use std::rc::Rc;

pub struct Context {
    before: String,
    after: String,
}

pub struct Executor {
    pub names: PrefixTree<char, Rc<dyn Fn(Context, &mut Executor) -> Context>>,
}

impl Executor {
    pub fn execute(&mut self, program: String) -> String {
        let mut context = Context {
            before: "".into(),
            after: program,
        };
        let mut after = &context.after[..];
        loop {
            let mut after_chars = after.chars();
            match self.names.get_by_shortest_prefix(&mut after_chars) {
                Some(function) => {
                    let function = function.clone();
                    let new_after = after_chars.as_str().to_owned();
                    context.after = new_after;
                    context = function(context, self);
                    after = &context.after[..];
                }
                None => {
                    let mut after_chars = after.chars();
                    match after_chars.next() {
                        None => return context.before + after,
                        Some(next_char) => {
                            context.before.push(next_char);
                            after = after_chars.as_str()
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor() {
        let mut executor = Executor {
            names: PrefixTree::new(),
        };
        executor.names.insert(
            "nothing".chars(),
            Rc::new(|context: Context, _executor: &mut Executor| context) as _,
        );

        assert_eq!(executor.execute("abcnothingdef".into()), "abcdef");

        executor.names.insert(
            "reverse".chars(),
            Rc::new(|context: Context, _executor: &mut Executor| Context {
                before: context.after,
                after: context.before,
            }) as _,
        );

        assert_eq!(executor.execute("abcreversedef".into()), "defabc");
    }
}
