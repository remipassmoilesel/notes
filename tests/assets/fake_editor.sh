#!/usr/bin/env bash

# This file is used for integration tests, as a fake editor

set -x
set -e

FILE=$1

echo "### File was just edited ###" > $FILE