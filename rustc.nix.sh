#! /usr/bin/env nix-shell
#! nix-shell -i bash -p pkg-config openssl libiconv darwin.apple_sdk.frameworks.Security darwin.apple_sdk.frameworks.SystemConfiguration darwin.apple_sdk.frameworks.Foundation darwin.apple_sdk.frameworks.AppKit curl libgpgerror gpgme

$@
