use tokio::io::{stdin, AsyncBufReadExt, BufReader};

use colored::*;

struct PromptOptions<'a> {
    prompt: &'a str,
    hint: Option<&'a str>,
    color: Option<&'a str>,
}

impl<'a> PromptOptions<'a> {
    #[allow(dead_code)]
    pub fn new(prompt: &'a str, hint: Option<&'a str>, color: Option<&'a str>) -> Self {
        Self {
            prompt,
            hint,
            color,
        }
    }

    pub fn yes_no(prompt: &'a str, default_yes: bool, color: Option<&'a str>) -> Self {
        let hint = Some(if default_yes {
            "[Yes / no]"
        } else {
            "[yes / No]"
        });

        Self {
            prompt,
            hint,
            color,
        }
    }
}

async fn do_prompt(options: PromptOptions<'_>) -> anyhow::Result<String> {
    let color = options.color.unwrap_or("normal");
    let prompt = match options.hint {
        Some(hint) => format!("{} {}", options.prompt, hint),
        None => options.prompt.to_owned(),
    };

    // print!() or write_all() and flush() don't seem to work
    //print!("{}", prompt.color(color));
    /*stdout()
        .write_all(format!("{} ", prompt.color(color)).as_bytes())
        .await?;
    stdout().flush().await?;*/
    println!("{}", prompt.color(color));

    let mut input = String::new();
    let mut reader = BufReader::new(stdin());
    reader.read_line(&mut input).await?;
    Ok(input)
}

#[allow(dead_code)]
pub async fn prompt_yes_no<S>(prompt: S, default_yes: bool) -> anyhow::Result<bool>
where
    S: AsRef<str>,
{
    let options = PromptOptions::yes_no(prompt.as_ref(), default_yes, None);
    Ok(match do_prompt(options).await?.to_lowercase().trim() {
        "yes" | "y" => true,
        "" => default_yes,
        _ => false,
    })
}

#[allow(dead_code)]
pub async fn prompt_error_yes_no<S>(prompt: S, default_yes: bool) -> anyhow::Result<bool>
where
    S: AsRef<str>,
{
    let options = PromptOptions::yes_no(prompt.as_ref(), default_yes, Some("red"));
    Ok(match do_prompt(options).await?.to_lowercase().trim() {
        "yes" | "y" => true,
        "" => default_yes,
        _ => false,
    })
}
