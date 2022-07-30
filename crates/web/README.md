# Web

## Development

```shell
# start web server and watch soucce files
cargo watch -q -c -w src/ -x run

# start tests
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```
