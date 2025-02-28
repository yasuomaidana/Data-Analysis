# Updating LibTorch

Here you can find some instructions and annotation which could be handy when updating LibTorch on macOS.

1. Check your current version of your package and how they are pointed in your `.zshrc` or `.bashrc` file:
    - Libomp
    - LibTorch
2. Go to the [PyTorch website](https://pytorch.org/get-started/locally/) and download the latest version of LibTorch.
3. Unzip the downloaded file and move the folder to the desired location.
4. Move the downloaded LibTorch to your environment LibTorch directory.
5. Ensure you removed the quarantine flag by running `xattr -r -d com.apple.quarantine $LIBTORCH_LIBRARY`

## Notes

1. I noticed that configuring `LIBTORCH_INCLUDE` generated errors, therefore I only configured two variables:
    - `LIBTORCH`
    - `LIBTORCH_LIBRARY`
2. You may also want to check these flags values:
    - `LDFLAGS`
    - `CPPFLAGS`
    - `DYLD_LIBRARY_PATH`

## My .zshrc file

```zsh
export LIBTORCH="$HOME/.libtorch"
export LIBTORCH_LIBRARY="$LIBTORCH/lib"


export DYLD_LIBRARY_PATH="$LIBTORCH_LIBRARY:$DYLD_LIBRARY_PATH"

export LIBOMP_LIB="/opt/homebrew/Cellar/libomp/19.1.7/lib"
export DYLD_LIBRARY_PATH="$LIBOMP_LIB:$DYLD_LIBRARY_PATH"
```
