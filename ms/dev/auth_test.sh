#!/bin/sh

mode="$1"
ci="ci"

if [ "$mode" = "$ci" ]; then
    cd /go/ms/auth/src && go test -v ./...
else 
    cd /go/ms/auth/src && go test -v ./... -coverprofile=/go/ms/dev/coverage/auth_coverage.out && go tool cover -html=/go/ms/dev/coverage/auth_coverage.out -o /go/ms/dev/coverage/auth.html
fi