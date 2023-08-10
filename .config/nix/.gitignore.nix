{ pkgs }:

let
  createDotGitignore = content: pkgs.writeText ".gitignore" content;
in
  createDotGitignore ''
    /target
    /node_modules
    .direnv
    result  
  ''
