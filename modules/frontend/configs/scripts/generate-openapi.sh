#!/bin/bash

yarn openapi-ts --client=@hey-api/client-axios -i http://localhost:8080/api/openapi -o ./src/generated/client