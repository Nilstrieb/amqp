use super::{snake_case, Amqp};
use heck::ToUpperCamelCase;

pub(super) fn codegen_random(amqp: &Amqp) {
    println!(
        "#[cfg(test)]
mod random {{
use rand::Rng;
use crate::methods::tests::RandomMethod;
use super::*;
"
    );

    impl_random("Method", || {
        let class_lens = amqp.classes.len();
        println!("        match rng.gen_range(0u32..{class_lens}) {{");
        for (i, class) in amqp.classes.iter().enumerate() {
            let class_name = class.name.to_upper_camel_case();
            println!("            {i} => {{");

            let method_len = class.methods.len();
            println!("                match rng.gen_range(0u32..{method_len}) {{");

            for (i, method) in class.methods.iter().enumerate() {
                let method_name = method.name.to_upper_camel_case();
                println!("                    {i} => Method::{class_name}{method_name} {{");
                for field in &method.fields {
                    let field_name = snake_case(&field.name);
                    println!("                        {field_name}: RandomMethod::random(rng),");
                }
                println!("                    }},");
            }
            println!(
                "                    _ => unreachable!(),
                }}"
            );

            println!("            }}");
        }
        println!(
            "            _ => unreachable!(),
        }}"
        );
    });

    println!("}}");
}

fn impl_random(name: &str, body: impl FnOnce()) {
    println!(
        "impl<R: Rng> RandomMethod<R> for {name} {{
    #[allow(unused_variables)]
    fn random(rng: &mut R) -> Self {{"
    );

    body();

    println!("    }}\n}}");
}
