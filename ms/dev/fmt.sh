#!/bin/sh

container_type="$1"

gopher="gopher"
rustacean="rustacean"
deno="deno"

if [ "$container_type" = "$gopher" ]; then
    cd /go/ms/auth/src && go fmt ./...
elif [ "$container_type" = "$rustacean" ]; then
    rustup component add rustfmt && \
    cd /ms/support/support && \
    cargo fmt && \ 
    cd /ms/support/support/lib && \
    cargo fmt
elif [ "$container_type" = "$deno" ]; then
    cd /ms/ui && deno task check
else
    echo "Invalid container_type $container_type must be gopher, rustacean or deno"
fi