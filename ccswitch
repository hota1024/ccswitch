#!/bin/bash
function ccswitch() {
    ccswitch_be $@
    export CLAUDE_CONFIG_DIR="$(cat /tmp/ccswitch_be)"
    ccswitch_be --success-to-switch
}
