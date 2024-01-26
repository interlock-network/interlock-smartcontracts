#!/usr/bin/env bash

# a stack overflow copypasta production...
# ...generate markdown file that links content from other markdown files
perl -ne 's#^!\[\[(.+?)\]\].*#`'$0' "$1"`#e;print' "$@"
