#!/bin/bash


# Make sure the Coppeliasim is running

consts_json_file="consts.json"

check_vars(){
        local vars=("RUST_ZMQREMOTEAPI" "COPPELIASIM_EDU_DIR")
        local vars_ok="true"
        for var in "${vars[@]}"  ; do
                if [ -z "${!var}" ]; then
                    echo "$var is not set."
                    local vars_ok="false"
                fi 
        done
        
        if [ $vars_ok = "false" ]; then
                exit 1 
        fi
}

download_deps(){

        echo "Downloading dependences"
        
        cd "$RUST_ZMQREMOTEAPI/c_transpiler/" || exit 1
        
        python -m venv .env
        source .env/bin/activate
        pip install -r ./requirements.txt

}

get_constants_from_simulator(){

        echo "getting constants from CoppeliaSim"
        
        cd "$COPPELIASIM_EDU_DIR/programming/zmqRemoteApi/tools" || exit 1
        
        python get_constants.py $consts_json_file || exit 1


}
update_rust_constants(){

        echo "updating the constants"
        
        local target_file="$RUST_ZMQREMOTEAPI/c_transpiler/assets/$consts_json_file"

        rm -f "$target_file"
        mv "$consts_json_file" "$target_file"
        
        
        python "$RUST_ZMQREMOTEAPI/c_transpiler/const_json_to_rust.py" || exit 1

}

download_deps
get_constants_from_simulator
update_rust_constants


echo "updated ./assets/sim_const.rs values"
