use std::io::{self, BufRead, Cursor, Read, Write};

use anyhow::{Context, Error};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "sentry-toolz")]
enum Cli {
    /// Decode a pickled-and-base64'd value from stdin and output it as JSON.
    ///
    /// This is how projectoptions are stored in Sentry.
    Decode,

    /// Pickle-and-base64 a JSON value from stdin.
    ///
    /// This is how projectoptions are stored in Sentry.
    Encode {
        #[structopt(short, long)]
        proto3: bool,
    },
    /// Send metrics from stdin into sentry project, using SENTRY_DSN envvar
    SendMetrics,
}

struct GarbageRemovingReader<R: Read>(R);

impl<R: Read> Read for GarbageRemovingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let num = self.0.read(buf)?;

        let mut i = 0;
        for j in 0..num {
            if buf[j].is_ascii_whitespace() || buf[j] == b'"' {
                continue;
            }

            buf[i] = buf[j];
            i += 1;
        }

        Ok(i)
    }
}

fn warn(msg: &str) {
    // logging is too hard
    eprintln!("\x1b[0;33mwarning:\x1b[0m {}", msg);
}

fn main() -> Result<(), Error> {
    let opt = Cli::from_args();

    match opt {
        Cli::Decode => {
            let stdin = io::stdin();
            let mut write = io::stdout();

            for line in stdin.lock().split(b'\n') {
                let mut read = GarbageRemovingReader(Cursor::new(line?));
                let data_pickled = base64::read::DecoderReader::new(&mut read, base64::STANDARD);

                // drive serializer manually such that we can catch errors about trailing data
                let mut data_deserializer = serde_pickle::Deserializer::new(data_pickled, true);
                let data_value: serde_json::Value =
                    serde::Deserialize::deserialize(&mut data_deserializer)?;
                if data_deserializer.end().is_err() {
                    warn("Trailing data while unpickling");
                }

                serde_json::to_writer(&mut write, &data_value)?;
                write.write(&[b'\n'])?;
            }
        }

        Cli::Encode { proto3 } => {
            let stdin = io::stdin();
            let mut write = io::stdout();

            for line in stdin.lock().split(b'\n') {
                let data_value: serde_json::Value = serde_json::from_reader(Cursor::new(line?))?;

                {
                    let mut write_base64 =
                        base64::write::EncoderWriter::new(&mut write, base64::STANDARD);
                    serde_pickle::to_writer(&mut write_base64, &data_value, proto3)?;
                }

                write.write(&[b'\n'])?;
            }
        }
        Cli::SendMetrics => {
            use sentry::metrics::Metric;

            // uses envvarn
            let _guard = sentry::init(());
            for (i, line) in std::io::stdin().lines().enumerate() {
                let line = line.context("failed to read line")?;
                let line = line.trim_end();
                Metric::parse_statsd(&line)
                    .context(format!("failed to parse statsd metric at line {}", i))?
                    .send();
            }
        }
    }

    Ok(())
}
