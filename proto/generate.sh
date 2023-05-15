#!/bin/bash -u
# Copyright 2019 dfuse Platform Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && cd .. && pwd )"

function main() {
  checks

  set -e

  pushd "$ROOT" >/dev/null
    echo "Generating Protobuf bindings via 'buf'"
    buf generate proto
  popd >/dev/null

  echo "Generated `date` by `whoami`" > $ROOT/proto/last_generate.txt
  echo "Commit `GIT_DIR=$ROOT/.git git rev-parse HEAD`" >> $ROOT/proto/last_generate.txt

  echo "Done"
}

function checks() {
  result=`printf "" | buf --version 2>&1 | grep -Eo '1\.(1[0-9]+|[2-9][0-9]+)\.'`
  if [[ "$result" == "" ]]; then
    echo "The 'buf' binary is either missing or is not recent enough (at `which buf || echo N/A`)."
    echo ""
    echo "To fix your problem, on Mac OS, perform this command:"
    echo ""
    echo "  brew install bufbuild/buf/buf"
    echo ""
    echo "On other system, refers to https://docs.buf.build/installation"
    echo ""
    echo "If everything is working as expetcted, the command:"
    echo ""
    echo "  buf --version"
    echo ""
    echo "Should print '1.11.0' (or newer)"
    exit 1
  fi
}

main "$@"
