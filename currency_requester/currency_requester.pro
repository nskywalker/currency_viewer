TEMPLATE = lib

CONFIG(debug, debug|release): system(cargo build)
CONFIG(release, debug|release): system(cargo build --release)

