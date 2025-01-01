#!/bin/sh

container_type="$1"

gopher="gopher"
rustacean="rustacean"
deno="deno"

if [ "$container_type" = "$gopher" ]; then
    mkdir -p /go/ms/dev/coverage
    cd /go/ms/auth/src && go test -v ./... -coverprofile=/go/ms/dev/coverage/auth_coverage.out && go tool cover -html=/go/ms/dev/coverage/auth_coverage.out -o /go/ms/dev/coverage/auth.html
elif [ "$container_type" = "$rustacean" ]; then
    cd /ms/support/support && cargo test --all --verbose
elif [ "$container_type" = "$deno" ]; then
    cd /ms/ui
else
    echo "Invalid container_type $container_type must be gopher, rustacean or deno"
fi