#!/bin/bash

# Get the directory of the script
SCRIPT_DIR="$(cd $(dirname ${BASH_SOURCE[0]}) && pwd)"

# Start a new Bash shell
PATH="$SCRIPT_DIR/target/debug:$SCRIPT_DIR/extras:$PATH" bash --rcfile <(echo "source $HOME/.bashrc; export PS1=\"\e[1;33m(aiutils-sh)\e[0m \e[1;34m\w\e[0m$ \"")
