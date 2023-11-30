#!/bin/bash

protoc \
  -I./proto/proto \
  -I./proto/third_party \
  --include_imports \
  --include_source_info \
  --descriptor_set_out=./target/backend.pb \
  --openapiv2_out=../empty-frontend/packages/openapi/openapi/ \
  ./proto/proto/lottery/lottery.proto \
  ./proto/proto/lottery/record.proto \
  ./proto/proto/lottery/template.proto \
  ./proto/proto/lottery/favorite.proto \
  ./proto/proto/auth/auth.proto \
  ./proto/proto/wx/wx.proto \
  ./proto/proto/wx/user.proto \
  ./proto/proto/health/department.proto \
  ./proto/proto/health/hospital.proto \
  ./proto/proto/health/nurse.proto \
  ./proto/proto/health/patient.proto \
  ./proto/proto/oss/oss.proto \
  ./proto/proto/user/user.proto \
  ./proto/proto/user/auth.proto \
  ./proto/proto/blog/blog.proto