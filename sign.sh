#!/bin/bash

osslsigncode sign \
  -pkcs12 ./assets/certs/pfx.pem \
  -n      "SGB_Initializer" \
  -i      "https://github.com/mil-acc-github/sgb_initializer" \
  -in     ./target/x86_64-pc-windows-msvc/release/sgb_initializer.exe \
  -out    ./.releases/latest/sgb_initializer.exe \
  -t      http://timestamp.digicert.com
