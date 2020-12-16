set -ex

main() {
  rustup self update

  if [ $TRAVIS_OS_NAME = osx ]; then
    export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib"
  else
    export DISPLAY=":99.0"
    Xvfb :99 -screen 0 800x600x24 > /dev/null 2>&1 &
  fi
}

main
