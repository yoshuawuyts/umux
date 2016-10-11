# µmux
Micro multiplexer

## Usage
```txt
umux - micro multiplexer

Options:
  -h, --help        Output usage information
  -v, --version     Output version number

Commands:
  (default)         Multiplex a local session
```

## Example
```txt
www-choo v1.0.0                                      1  [ 2 ]  3   4   5   6
[0000] info [client] at http://127.0.0.1:8080
[0000] info [browserify] 12ms  11kb
[0000] info [server]      7ms   2kb GET 200 /
[0000] info [server]      3ms  12kb GET 200 /bundle.js
[0000] info [server]      2ms   4kb GET 200 /bundle.css
[0002] info [browserify] 14ms  12kb
```

## Config
```toml
# ~/.config/umux.toml
[windows]
buildkite = "/usr/bin/buildkite-mux builds"
kube = "/usr/bin/kube-mux overview"

[[language]]
name = "node"
windows = [ "overview", "stdout", "stderr", "buildkite", "kube" ]
```

## Installation
```sh
$ cargo install umux
```

## License
[MIT](https://tldrlegal.com/license/mit-license)
