use std::io;

use structopt::StructOpt;
use anyhow::Error;


#[derive(StructOpt)]
#[structopt(name = "sentry-toolz")]
enum Cli {
    Decode,
    Encode {
        #[structopt(short, long)]
        proto3: bool
    }
}

fn main() -> Result<(), Error> {
    let opt = Cli::from_args();

    match opt {
        Cli::Decode => {
            let mut read = io::stdin();
            let data_pickled = base64::read::DecoderReader::new(
                &mut read,
                base64::STANDARD
            );

            let mut write = io::stdout();

            let data_value: serde_json::Value = serde_pickle::from_reader(data_pickled)?;
            serde_json::to_writer(&mut write, &data_value)?;
        },

        Cli::Encode { proto3 } => {
            let mut read = io::stdin();
            let mut write = io::stdout();
            let mut write_base64 = base64::write::EncoderWriter::new(&mut write, base64::STANDARD);

            let data_value: serde_json::Value = serde_json::from_reader(&mut read)?;
            serde_pickle::to_writer(&mut write_base64, &data_value, proto3)?;
        }
    }

    Ok(())
}
