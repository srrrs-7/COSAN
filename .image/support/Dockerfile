FROM golang:latest AS builder

ADD ./ms /go/ms
WORKDIR /go/ms/support/src

ARG GOOS=linux
ARG GOARCH=amd64
ARG CGO_ENABLED=false
RUN go build -ldflags="-s -w" -o /go/bin/api ./cmd/main.go

FROM scratch
COPY --from=builder /go/bin/api /usr/local/bin/api

EXPOSE 8080

CMD [ "/usr/local/bin/api" ]