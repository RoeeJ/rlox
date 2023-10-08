#!/bin/bash
npx nodemon -w src -w ./tests/* -e '*' --exec "reset && cargo test -q -- || exit 1"
