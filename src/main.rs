use std::io::Write;

fn main() {
    let tmpfile = std::env::temp_dir().join(format!("rusti_{}.rs", std::process::id()));
    let tmpexec = tmpfile.with_extension("");
    let mut commands: Vec<String> = vec![];
    println!("Welcome to rust interpreter!");
    println!("Saving state to {}", tmpfile.display());
    println!("------------------------------");
    loop {
        let mut input = String::new();

        print!("rusti > ");
        let _ = std::io::stdout().flush();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut code = commands.join("\n");
        if let Some((lhs, _)) = input.split_once('=') {
            let identifier = lhs.split_whitespace().last().unwrap();
            code.push_str(format!("\n{input};\nprintln!(\"{{:?}}\", {identifier});").as_str());
        } else {
            code.push_str(format!("\nlet result = {input};\nprintln!(\"{{:?}}\", result);").as_str());
        }

        code = format!("fn main() {{\n{code}\n}}");
        std::fs::write(&tmpfile, code.as_bytes()).unwrap();

        // System call
        let out = std::process::Command::new("rustc")
            .arg(&tmpfile)
            .arg("-o")
            .arg(&tmpexec)
            .output();

        if let Ok(out) = out {
            let output = if out.status.success() {
                let out = std::process::Command::new(&tmpexec).output().unwrap();

                if out.status.success() {
                    commands.push(format!("{input};"));
                    out.stdout
                } else {
                    out.stderr
                }
            } else {
                out.stderr
            };

            println!("{}", String::from_utf8(output).unwrap());
        } else {
            println!("{out:?}");
            break;
        }
    }
}
