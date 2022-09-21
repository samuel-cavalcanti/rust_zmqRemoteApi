# C Transpiler

this Transpiler converts the C++ functions declaration
to my macro [requests!](../src/remote_api_objects/api_objects.rs#106).

run tests:

```bash
python -m unittest discover  -v tests
```

Python version: **3.10.7**

## Converting the C++ code to the rust macro

Make sure that you are in this current directory and you have installed the [requirements.txt](requirements.txt)

to install the requests

```bash
pip install -r requirements.txt 
```

run the script

```bash
python cpp_to_rust.py
```
