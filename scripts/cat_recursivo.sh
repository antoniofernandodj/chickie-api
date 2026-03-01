#!/bin/bash

DIR="${1:-.}"

find "$DIR" -type f | sort | while read -r arquivo; do
    echo "===== $arquivo ====="
    cat "$arquivo"
    echo ""
done
