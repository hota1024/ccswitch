#!/bin/bash
function ccswitch() {
    ./target/debug/ccswitch_be $@
    export CLAUDE_CONFIG_DIR="$(cat /tmp/ccswitch_be)"
    ./target/debug/ccswitch_be --success-to-switch
}
