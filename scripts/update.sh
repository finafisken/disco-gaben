#!/bin/bash

VERSION=$1

# download release
wget https://github.com/finafisken/disco-gaben/releases/download/v$VERSION/disco-gaben-v$VERSION.tar.gz

# extract into folder
tar xzvf disco-gaben-v$VERSION.tar.gz bin

# copy .env file
cp .env bin/.env

# restart supervisor daemon
sudo systemctl restart supervisord