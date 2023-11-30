# Judge A Book
### Query Book.io collections for their covers

This is a demo project for interfacing with the Book.io API and associated
blockchain and distributed asset storage solution. At the time of writing,
the blockchain is Cardano, the interface to the chain is the Blockfrost, and
book assets such as content and cover images are stored in IPFS.

The project implements a CLI for querying the API for a given Book.io collection
ID (Cardano Policy ID) and supplying an output directory, validating the ID
represents a valid Book.io collection, and retrieves the specified number of cover
images for books associated with the collection. Per the project template, the CLI will
attempt to download 10 cover images of the "High-Resolution" variant but this has
been implemented as default values of configurable extra flags supplied at the command
line. The project has been structured in such a way as to allow for theoretical
extensions such as adding additional commands (add command variants to the `clap`
subcommand enum and implement the subcommand's `run` method), supply global
configuration via environment variables, and encapsulate the interactions with all
blockchain and distributed storage services via REST APIs queried via `reqwest`.
The implementation aspires to runtime simplicity and speed, opting for blocking
REST calls but running them in parallel via `rayon`.

To run the project, install rust `1.72.0` or greater, clone the project and build the
CLI with `cargo build --release`. The `judge` binary will be waiting for you at the
end in the `./target/release` directory.

Test the tool with the command:
```
./target/release/judge fetch-covers \
    -c, --collection aa21582ec1ce92f2c21ac61c8b5bbcdaadad0efa59aca5d64fba22ab \
    -o, --outdir ./output \
    --api-key <BLOCKFROST PROJECT KEY>
```

This will output 10 cover images to the `./output` directory in the format
`aa21582ec1ce92f2c21ac61c8b5bbcdaadad0efa59aca5d64fba22ab-high-<CID>.png`.
Additional flags are `-n, --number` to adjust the number of images downloaded,
`-r, --res` to swap the hi-res cover image for the standard image, and `--chain-base-url`
and `--asset-base-url` to customize the source of the blockchain and asset storage
endpoints. In addition, the environment variables `JUDGE_API_KEY`, `JUDGE_CHAIN_URL`, and
`JUDGE_ASSETS_URL` can be set to avoid passing the Blockfrost project key or any custom
URLs at each command input.

The tool attempts to download only as many image covers as requested per input collection
ID, resolution, and output directory and to do so only once idempotently. For example, the
above command, uninterrupted, will download 10 cover images from the `aa21582ec1ce92f2c21ac61c8b5bbcdaadad0efa59aca5d64fba22ab`
collection in high-resolution format to the `./output` directory and subsequent runs of the
command will exit immediately without re-downloading any images. Deleting a single image and
re-running again will download one additional cover image. Increasing the number of requested
images will result in the difference being downloaded.