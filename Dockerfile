FROM etrombly/rust-crosscompile:latest

#
# Build and package executable
#
CMD ["/usr/bin/package.sh"]
