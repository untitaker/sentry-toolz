# sentry-toolz

Small tool to decode/encode projectoptions in
[Sentry](https://github.com/getsentry/sentry), which are pickled and
base64-encoded Python primitives.

The point of writing this in Rust is to limit attack surface of pickle.

## License

Licensed under the 3-clause BSD license, see ``LICENSE``.
