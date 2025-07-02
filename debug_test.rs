use graph_generation_lang::GGLEngine;

fn main() {
    let mut engine = GGLEngine::new();

    let ggl_code = r#"
        graph test {
            node alice :person [name="Alice", age=30];
            node bob :person [name="Bob", age=25];
        }
    "#;

    let result = engine.generate_from_ggl_native(ggl_code);
    match result {
        Ok(json_str) => {
            println!("Generated JSON:");
            println!("{}", json_str);

            let graph: serde_json::Value = serde_json::from_str(&json_str).unwrap();
            println!("\nParsed JSON:");
            println!("{:#}", graph);

            println!("\nAlice age value:");
            println!("{:?}", graph["nodes"]["alice"]["metadata"]["age"]);
        }
        Err(e) => println!("Error: {}", e),
    }
}
