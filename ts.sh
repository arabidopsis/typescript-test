#!/bin/bash
cmd=$1
exec cargo run -- --first --cmd "$cmd" | prettier --parser typescript | bat -l typescript
