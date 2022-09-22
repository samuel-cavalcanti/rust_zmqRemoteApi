# C Transpiler

this Transpiler converts the C++ functions declaration
to macro [requests!](../src/remote_api_objects/sim/sim.rs#8) input and create python protocol classes [sim.py](assets/sim.py) and [sim_ik](assets/sim_ik.py).

run tests:

```bash
python -m unittest discover  -v tests
```

Python version: **3.10.7**

## Converting the C++ code to the rust macro and python Protocol

Make sure that you are in this current directory and you have installed the [requirements.txt](requirements.txt)

to install the requests

```bash
pip install -r requirements.txt 
```

run the script

```bash
python convert.py
```
