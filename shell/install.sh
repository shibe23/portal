#!/bin/bash
echo source `pwd`/shell/portal.sh >> ~/.bashrc
ln -s `pwd`/target/release/portal /usr/local/bin/portal-exec