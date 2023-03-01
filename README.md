# sentry-toolz

Small tool to decode/encode projectoptions in
[Sentry](https://github.com/getsentry/sentry), which are pickled and
base64-encoded Python primitives.

The point of writing this in Rust is to limit attack surface of pickle.

## Installation

```bash
make install  # install all tools into ~/.local/bin/
```

## Decoding projectoption value

```bash
echo 1 | sentry-toolz encode
gAJKAQAAAC4=

echo 'gAJKAQAAAC4=' | sentry-toolz decode
1

echo 1 > values.txt
echo 2 >> values.txt
echo 3 >> values.txt

# each line is its own value. whitespace and garbage is ignored. this makes it
# easy to pipe CSVs or output from postgres into sentry-toolz
cat values.txt | sentry-toolz encode
gAJKAQAAAC4=
gAJKAgAAAC4=
gAJKAwAAAC4=
```

## Uploading data into Sentry

**NOTE**: Some of those tools require the Python SDK to be importable!

```bash

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
