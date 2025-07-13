#!/bin/bash
# Copyright (c) godot-rust; Bromeon and contributors.
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.


ALL_ARGS=("$@")
CMD=("${ALL_ARGS[@]:-1}")
WARNING_MESSAGE="${ALL_ARGS[-1]}"

if "${CMD[@]}"; then
    echo "Command executed successfully."
else
  echo "::warning ::" "$WARNING_MESSAGE"
fi
