# C Transpiler

this Transpiler converts the C++ functions declaration
to macro [requests!](../src/remote_api_objects/sim/sim.rs#8) input and create python protocol classes [sim.py](assets/sim.py) and [sim_ik](assets/sim_ik.py).

Run tests:

```bash
python -m unittest discover  -v tests
```

Python version: **3.10.7**

## Converting the C++ code to the rust macro and python Protocol

Make sure that you are in this current directory and you have installed the [requirements.txt](requirements.txt)

Install the requests

```bash
pip install -r requirements.txt
```

Run the script

```bash
python convert.py
```

## Updating for new versions of CoppeliaSim

### updating constants values

First run the simulator, and set 2 variables:

- `RUST_ZMQREMOTEAPI`: this repository directory
- `COPPELIASIM_EDU_DIR`: the Coppeliasim directory

Then run the script [`./get_consts.sh`](./get_consts.sh)

### update the cpp headers

Look into the headers and get the functions declarations from `RemoteAPIObjects.h`,`sim-deprecated.h` and `sim-special.h`, put all functions
to [`assets/sim_api_header.h`](./assets/sim_api_header.h)

Run the tests.
