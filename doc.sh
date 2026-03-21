#!/usr/bin/env bash

# *brakoll - d: serve cargo doc on server, p: 0, t: tooling, s: closed
cargo doc

port=7002
crate_name=$(git remote get-url origin | sed 's#.*/##; s#.git##')
docdir="./target/doc"

lsof -ti tcp:$port | xargs -r kill -9
python3 -m http.server $port -d "$docdir" >/dev/null 2>&1 &
echo "http://127.0.0.1:$port/$crate_name/"

