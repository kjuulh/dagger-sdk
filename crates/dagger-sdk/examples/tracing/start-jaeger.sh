#!/bin/bash

set -e

docker run --name jaeger -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest
