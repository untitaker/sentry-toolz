use std::io::{self, Read, Write};

use anyhow::Error;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "sentry-toolz")]
enum Cli {
    Decode,
    Encode {
        #[structopt(short, long)]
        proto3: bool,
    },
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
            let mut read = GarbageRemovingReader(io::stdin());
            let data_pickled = base64::read::DecoderReader::new(&mut read, base64::STANDARD);

            // drive serializer manually such that we can catch errors about trailing data
            let mut data_deserializer = serde_pickle::Deserializer::new(data_pickled, true);
            let data_value: serde_json::Value =
                serde::Deserialize::deserialize(&mut data_deserializer)?;
            if data_deserializer.end().is_err() {
                warn("Trailing data while unpickling");
            }

            let mut write = io::stdout();
            serde_json::to_writer(&mut write, &data_value)?;
            write.write(&[b'\n'])?;
        }

        Cli::Encode { proto3 } => {
            let mut read = io::stdin();
            let mut write = io::stdout();
            {
                let mut write_base64 =
                    base64::write::EncoderWriter::new(&mut write, base64::STANDARD);

                let data_value: serde_json::Value = serde_json::from_reader(&mut read)?;
                serde_pickle::to_writer(&mut write_base64, &data_value, proto3)?;
            }
            write.write(&[b'\n'])?;
        }
    }

    Ok(())
}
