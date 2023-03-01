use argh::FromArgs;
use futures_lite::future::block_on;
use melstructs::{Address, NetID};

#[derive(FromArgs, PartialEq, Debug)]
/// Look up a name in the Gibbername registry.
struct Cli {
    #[argh(subcommand)]
    command: Command,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Command {
    Lookup(Lookup),
    Register(Register)
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "lookup")]
/// Lookup what is bound to a name
struct Lookup {
    #[argh(positional)]
    name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "register")]
/// Register a name
struct Register {
    #[argh(option, description = "the Mel address of the gibbername owner")]
    owner: Address,

    #[argh(option, description = "the data to be bound to the gibbername")]
    binding: String
}

fn main() -> anyhow::Result<()> {
    let args: Cli = argh::from_env();
    // keep around a client
    let client = block_on(
        melprot::Client::autoconnect(NetID::Testnet)
    )?;

    match args.command {
        Command::Lookup(lookup) => {
            // we don't need a futures runtime, block_on is fine
            let gname = block_on(gibbername::lookup(&client, &lookup.name))?;
            println!("{gname}");
        }
        Command::Register(register) => {
            // gibbername will prompt the user
            let name = block_on(gibbername::register(&client, register.owner, &register.binding))?;
            println!("registered {:?}", name);
        }
    };
    Ok(())
}