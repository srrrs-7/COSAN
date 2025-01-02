#!/bin/bash

key=$(openssl genrsa 2048 | sed -n '10p' | tr '/' '-')
cd ../../
sed "s/SECRET_KEY:.*/SECRET_KEY: $(echo $key)/g" compose.override.yaml > compose.override.yaml.tmp
mv compose.override.yaml.tmp compose.override.yaml