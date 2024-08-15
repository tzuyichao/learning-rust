#!/bin/bash

find . -type f -executable -exec file {} \; | grep 'ELF' | cut -d: -f1 | xargs rm
