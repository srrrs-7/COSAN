#!/bin/sh

mode="$1"
ci="ci"

if [ "$mode" = "$ci" ]; then
    cd /go/ms/auth/src && go test -v ./...
else 
    mkdir -p /go/ms/dev/coverage
    cd /go/ms/auth/src && go test -v ./... -coverprofile=/go/ms/dev/coverage/auth_coverage.out && go tool cover -html=/go/ms/dev/coverage/auth_coverage.out -o /go/ms/dev/coverage/auth.html
fi