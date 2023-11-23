#!/bin/bash

docs=(
  user/user
  user/auth
)

types=(
  fetch
  axios
)

for doc in "${docs[@]}"; do
  protoc \
    -I. \
    -I../third_party \
    --openapiv2_out=../../../empty-frontend/packages/openapi/openapi/ \
    ./$doc.proto
done

cd  ../../../empty-frontend/packages/openapi

for doc in "${docs[@]}"; do
  for type in "${types[@]}"; do
    pnpm exec openapi-generator-cli generate -g typescript-$type \
    -i ./openapi/$doc.swagger.json \
    -o ./src/$type/$doc
  done
done
