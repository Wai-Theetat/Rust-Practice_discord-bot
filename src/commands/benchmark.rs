
use std::time::{Duration, Instant};

use serenity::builder::{
    CreateCommand, CreateCommandOption, EditInteractionResponse,
};
use serenity::model::application::{CommandInteraction, CommandOptionType};
use serenity::prelude::*;

use crate::math::factorization::factorize_with_config;

const MAX_RUNS: u32 = 5;
const ITERATION_BUDGET: usize = 1_000_000;
const MAX_ROUNDS: usize = 4;

pub fn register() -> CreateCommand {
    CreateCommand::new("benchmark")
        .description("Benchmark factorization performance.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "n",
                "Integer to factorize (u64)",
            )
            .required(true)
            .min_length(1)
            .max_length(20),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "runs",
                "Number of benchmark runs (1-5)",
            )
            .min_int_value(1)
            .max_int_value(5)
            .required(true),
        )
}

pub async fn start(ctx: &Context, command: &CommandInteraction) {
    let n = command
        .data
        .options
        .iter()
        .find(|option| option.name == "n")
        .and_then(|option| option.value.as_str())
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|&value| value >= 2);

    let runs = command
        .data
        .options
        .iter()
        .find(|option| option.name == "runs")
        .and_then(|option| option.value.as_i64())
        .and_then(|value| u32::try_from(value).ok())
        .filter(|&value| (1..=MAX_RUNS).contains(&value));

    let (Some(n), Some(runs)) = (n, runs) else {
        let _ = command
            .create_response(
                &ctx.http,
                serenity::builder::CreateInteractionResponse::Message(
                    serenity::builder::CreateInteractionResponseMessage::new()
                        .content("Invalid input. Check n and runs (1-5).")
                        .ephemeral(true),
                ),
            )
            .await;
        return;
    };

    if let Err(error) = command.defer(&ctx.http).await {
        eprintln!("Could not defer benchmark: {error}");
        return;
    }

    let result = tokio::task::spawn_blocking(move || {
        benchmark(n, runs)
    })
    .await;

    let message = match result {
        Ok(Ok(output)) => output,
        Ok(Err(error)) => format!("Benchmark failed: {error}"),
        Err(error) => format!("Benchmark task failed: {error}"),
    };

    if let Err(error) = command
        .edit_response(
            &ctx.http,
            EditInteractionResponse::new().content(message),
        )
        .await
    {
        eprintln!("Could not send benchmark results: {error}");
    }
}

struct BenchmarkResult {
    workers: usize,
    average: Duration,
}

fn benchmark(n: u64, runs: u32) -> Result<String, String> {
    let worker_counts = [1, 2, 4];
    let mut results = Vec::new();
    let mut expected_factors: Option<Vec<u64>> = None;

    for workers in worker_counts {
        let mut total = Duration::ZERO;

        for _ in 0..runs {
            let start = Instant::now();

            let factors = factorize_with_config(
                n,
                workers,
                ITERATION_BUDGET,
                MAX_ROUNDS,
            )
            .map_err(|error| error.to_string())?;

            let elapsed = start.elapsed();

            if let Some(expected) = &expected_factors {
                if expected != &factors {
                    return Err("Factorization results differ between runs.".to_string());
                }
            } else {
                expected_factors = Some(factors);
            }

            total += elapsed;
        }

        results.push(BenchmarkResult {
            workers,
            average: total / runs,
        });
    }

    let baseline = results[0].average.as_secs_f64();

    let mut output = format!(
        "## Benchmark Results\nInput: `{n}`\nRuns: `{runs}`\n\n"
    );

    output.push_str("```text\n");
    output.push_str("Workers   Average       Speedup\n");

    for result in &results {
        let average_ms = result.average.as_secs_f64() * 1000.0;
        let speedup = baseline / result.average.as_secs_f64();

        output.push_str(&format!(
            "{:<8} {:>9.2} ms    {:>6.2}x\n",
            result.workers,
            average_ms,
            speedup
        ));
    }

    output.push_str("```");

    let fastest = results
        .iter()
        .min_by_key(|result| result.average)
        .unwrap();

    output.push_str(&format!(
        "\nFastest configuration: **{} workers**",
        fastest.workers
    ));

    Ok(output)
}
