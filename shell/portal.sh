#!/bin/bash
function portal () {
  OUTPUT=`portal-exec $@`
  if [ $? -eq 2 ]
    then cd "$OUTPUT"
    else echo "$OUTPUT"
  fi
}