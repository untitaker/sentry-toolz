# sentry-toolz

Small tool to decode/encode projectoptions in
[Sentry](https://github.com/getsentry/sentry), which are pickled and
base64-encoded Python primitives.

The point of writing this in Rust is to limit attack surface of pickle.

## Uploading data into Sentry

**NOTE**: Some of those tools require the Python SDK to be importable!

```bash
make install  # install all tools into ~/.local/bin/

export SENTRY_DSN="$(dump-dsn 1)"  # get local server's DSN for project 1

# upload events
sentry-upload-event event.json event2.json ...

# upload minidumps
sentry-upload-minidump minidump.dmp

# upload random session
sentry-upload-session

# upload bogus stacktrace with function names a, b, c
sentry-upload-event <(build-stacktrace a b c)

```

## License

Licensed under the 3-clause BSD license, see ``LICENSE``.
