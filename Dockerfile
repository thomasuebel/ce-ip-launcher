FROM thomasuebel/fedora-mingw-rust:latest

#
# Build and package executable
#
CMD ["/usr/bin/package.sh"]
