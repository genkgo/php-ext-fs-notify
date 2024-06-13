dnl  Licensed to the Apache Software Foundation (ASF) under one
dnl  or more contributor license agreements.  See the NOTICE file
dnl  distributed with this work for additional information
dnl  regarding copyright ownership.  The ASF licenses this file
dnl  to you under the Apache License, Version 2.0 (the
dnl  "License"); you may not use this file except in compliance
dnl  with the License.  You may obtain a copy of the License at
dnl
dnl    http://www.apache.org/licenses/LICENSE-2.0
dnl
dnl  Unless required by applicable law or agreed to in writing,
dnl  software distributed under the License is distributed on an
dnl  "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
dnl  KIND, either express or implied.  See the License for the
dnl  specific language governing permissions and limitations
dnl  under the License.

PHP_ARG_ENABLE([fs_notify],
  [whether to enable fs_notify support],
  [AS_HELP_STRING([--enable-fs-notify],
    [Enable fs_notify support])],
  [no])

PHP_ARG_ENABLE([cargo_debug], [whether to enable cargo debug mode],
[  --enable-cargo-debug           Enable cargo debug], no, no)

if test "$PHP_THREAD_SAFETY" == "yes"; then
  AC_MSG_ERROR([fs_notify does not support ZTS])
fi

if test "$PHP_FS_NOTIFY" != "no"; then
  AC_PATH_PROG(CARGO, cargo, no)
  if ! test -x "$CARGO"; then
    AC_MSG_ERROR([cargo command missing, please reinstall the cargo distribution])
  fi

  AC_DEFINE(HAVE_FS_NOTIFY, 1, [ Have fs_notify support ])

  PHP_NEW_EXTENSION(fs_notify, [ ], $ext_shared)

  CARGO_MODE_FLAGS="--release"
  CARGO_MODE_DIR="release"

  if test "$PHP_CARGO_DEBUG" != "no"; then
    CARGO_MODE_FLAGS=""
    CARGO_MODE_DIR="debug"
  fi

  cat >>Makefile.objects<< EOF
all: cargo_build

clean: cargo_clean

cargo_build:
	PHP_CONFIG=$PHP_PHP_CONFIG cargo build $CARGO_MODE_FLAGS
	if [[ -f ./target/$CARGO_MODE_DIR/libphp_ext_fs_notify.dylib ]] ; then \\
		cp ./target/$CARGO_MODE_DIR/libphp_ext_fs_notify.dylib ./modules/fs_notify.dylib ; fi
	if [[ -f ./target/$CARGO_MODE_DIR/libphp_ext_fs_notify.so ]] ; then \\
		cp ./target/$CARGO_MODE_DIR/libphp_ext_fs_notify.so ./modules/fs_notify.so ; fi

cargo_clean:
	cargo clean

.PHONY: cargo_build cargo_clean
EOF

  AC_CONFIG_LINKS([ \
    Cargo.lock:Cargo.lock \
    Cargo.toml:Cargo.toml \
    build.rs:build.rs \
    src:src \
    tests:tests \
    ])
fi