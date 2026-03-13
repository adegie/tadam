use std::fs::File;
use std::io::{self, BufReader, Cursor, IsTerminal, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use rodio::{Decoder, OutputStreamBuilder, Sink};

const DEFAULT_SOUND: &[u8] = include_bytes!("../assets/gentle_breeze_and_birds_singing.ogg");

#[derive(Parser, Debug)]
#[command(name = "tadam")]
#[command(about = "Play a completion sound when piped input finishes")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(long, value_name = "FILE")]
    sound: Option<PathBuf>,

    #[arg(long, help = "Do not forward stdin to stdout")]
    quiet: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Completions {
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn main() {
    if let Err(err) = run() {
        eprintln!("tadam: {err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    if let Some(Commands::Completions { shell }) = cli.command {
        print_completions(shell);
        return Ok(());
    }

    if io::stdin().is_terminal() {
        anyhow::bail!("stdin is a terminal. Pipe a command into tadam, e.g. `ls -l | tadam`");
    }

    if !cli.quiet {
        forward_stdin_to_stdout().context("failed to read piped input")?;
    } else {
        consume_stdin().context("failed to read piped input")?;
    }

    play_sound(cli.sound.as_deref()).context("failed to play sound")?;
    Ok(())
}

fn print_completions(shell: Shell) {
    let mut command = Cli::command();
    generate(shell, &mut command, "tadam", &mut io::stdout());
}

fn forward_stdin_to_stdout() -> io::Result<()> {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    io::copy(&mut stdin, &mut stdout)?;
    stdout.flush()
}

fn consume_stdin() -> io::Result<()> {
    let mut stdin = io::stdin().lock();
    let mut sink = io::sink();
    io::copy(&mut stdin, &mut sink)?;
    Ok(())
}

fn play_sound(sound_path: Option<&std::path::Path>) -> Result<()> {
    let mut stream =
        OutputStreamBuilder::open_default_stream().context("no audio output device")?;
    stream.log_on_drop(false);
    let sink = Sink::connect_new(stream.mixer());

    match sound_path {
        Some(path) => {
            let file = File::open(path)
                .with_context(|| format!("could not open sound file `{}`", path.display()))?;
            let source = Decoder::new(BufReader::new(file)).context("unsupported sound format")?;
            sink.append(source);
        }
        None => {
            let cursor = Cursor::new(DEFAULT_SOUND);
            let source =
                Decoder::new(BufReader::new(cursor)).context("embedded sound file is invalid")?;
            sink.append(source);
        }
    }

    sink.sleep_until_end();
    drop(stream);
    Ok(())
}
