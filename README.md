# caress
A tool to touch a file while creating any necessary directories automatically.

## Install (Unix)
```sh
export VER=$(wget -qO- https://github.com/Gnarus-G/caress/releases/latest | grep -oP 'v\d+\.\d+\.\d+' | tail -n 1);
curl -L https://github.com/Gnarus-G/caress/releases/download/$VER/caress-$OSTYPE.tar.gz -o caress.tar.gz
tar -xzvf caress.tar.gz caress
# Allow to able to run it from anywhere [Optional]
sudo mv caress /usr/local/bin
```

## Usage
Grab a binary (linux or mac) in [releases](https://github.com/Gnarus-G/caress/releases)
```sh
./caress --help
```
## Demo
![caress-demo-2022-06-30_00 39 36](https://user-images.githubusercontent.com/37311893/176594383-e3723f73-f1c3-4dbe-9a70-71c4b6c21552.gif)
