#!/bin/bash

##################################################
# We call this from an Xcode run script.
##################################################

set -e

if [[ -z "$PROJECT_DIR" ]]; then
    echo "Must provide PROJECT_DIR environment variable set to the Xcode project directory." 1>&2
    exit 1
fi

cd $PROJECT_DIR

export PATH="$HOME/.cargo/bin:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:$PATH"

env

if [[ "$PLATFORM_NAME" = "macosx" ]]; then
	TARGETS="aarch64-apple-darwin"
	if [[ $CONFIGURATION == "Release" ]]; then
	    echo "BUIlDING FOR RELEASE ($TARGETS)"
	
	    cargo lipo --release --manifest-path ./wstunnel/wstunnel-cli/Cargo.toml  --targets $TARGETS
	else
	    echo "BUIlDING FOR DEBUG ($TARGETS)"
	
	    cargo lipo --manifest-path ./wstunnel/wstunnel-cli/Cargo.toml  --targets $TARGETS
	fi
else
	TARGETS="aarch64-apple-ios"
	if [[ $CONFIGURATION == "Release" ]]; then
	    echo "BUIlDING FOR RELEASE ($TARGETS)"
	
	    cargo lipo --release --manifest-path ./wstunnel/wstunnel-cli/Cargo.toml  --targets $TARGETS
	else
	    echo "BUIlDING FOR DEBUG ($TARGETS)"
	
	    cargo lipo --manifest-path ./wstunnel/wstunnel-cli/Cargo.toml  --targets $TARGETS
	fi
fi
