# toy_json_python

## Usage

```shell
python -m venv .venv
. .venv/bin/activate
pip install maturin
```

## Using from python

```shell
maturin develop
python
Python 3.10.4 (main, Jun 15 2022, 11:07:37) [Clang 13.1.6 (clang-1316.0.21.2.5)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> import toy_json_python
>>> print(toy_json_python.format("""{"key": "Hello, World"}""", indent=2))
{
  "key": "Hello, World"
}

>>>
```