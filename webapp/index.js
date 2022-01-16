import * as wasm from "wasm-passwords";

window.getPassword = function(){
    return wasm.get_password();
}

window.getRotatedPassword = function(){  
    return wasm.get_rotated_password(); 
}


