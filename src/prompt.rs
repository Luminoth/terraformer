use std::io::{stdin, stdout, Write};

use colored::*;

struct PromptOptions<'a> {
    prompt: &'a str,
    hint: Option<&'a str>,
    color: Option<&'a str>,
}

impl<'a> PromptOptions<'a> {
    pub fn new(prompt: &'a str, hint: Option<&'a str>, color: Option<&'a str>) -> Self {
        Self {
            prompt,
            hint,
            color,
        }
    }
}

fn do_prompt(options: PromptOptions) -> anyhow::Result<String> {
    let color = options.color.unwrap_or("normal");
    let prompt = match options.hint {
        Some(hint) => format!("{} {}", options.prompt, hint),
        None => options.prompt.to_owned(),
    };

    print!("{} ", prompt.color(color));
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input)
}

#[allow(dead_code)]
pub fn prompt_yes_no<S>(prompt: S, default_yes: bool) -> anyhow::Result<bool>
where
    S: AsRef<str>,
{
    let options = PromptOptions::new(
        prompt.as_ref(),
        Some(if default_yes {
            "[Yes / no]"
        } else {
            "[yes / No]"
        }),
        None,
    );

    Ok(match do_prompt(options)?.to_lowercase().trim() {
        "yes" | "y" => true,
        "" => default_yes,
        _ => false,
    })
}

#[allow(dead_code)]
pub fn prompt_error_yes_no<S>(prompt: S, default_yes: bool) -> anyhow::Result<bool>
where
    S: AsRef<str>,
{
    let options = PromptOptions::new(
        prompt.as_ref(),
        Some(if default_yes {
            "[Yes / no]"
        } else {
            "[yes / No]"
        }),
        Some("red"),
    );

    Ok(match do_prompt(options)?.to_lowercase().trim() {
        "yes" | "y" => true,
        "" => default_yes,
        _ => false,
    })
}
