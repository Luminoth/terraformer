use std::io::{stdin, stdout, Write};

pub fn prompt<S>(output: S) -> anyhow::Result<String>
where
    S: AsRef<str>,
{
    print!("{}", output.as_ref());
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input)
}

pub fn prompt_yes_no<S>(output: S, default_yes: bool) -> anyhow::Result<bool>
where
    S: AsRef<str>,
{
    let hint = if default_yes {
        "[Yes / no]"
    } else {
        "[yes / No]"
    };

    let output = format!("{} {}? ", output.as_ref(), hint);
    Ok(match prompt(output)?.to_lowercase().trim() {
        "yes" => true,
        "" => default_yes,
        _ => false,
    })
}
