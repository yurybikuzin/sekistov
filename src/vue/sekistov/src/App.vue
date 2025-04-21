<script setup lang="js">
import { onMounted, reactive, computed, ref } from 'vue'


const console_info = compound => console.log(strFromCompound(lib, compound))
const console_warn = compound => console.warn(strFromCompound(lib, compound))
const console_err = compound => console.error(strFromCompound(lib, compound))
const console_err_allocate = (size, compound) => size > 0 ? 
    console.error(`failed to allocate ${size} bytes in ${strFromCompound(lib, compound)}`) 
    : 
    console.error(`failed to allocate memmory in ${strFromCompound(lib, compound)}`)
let lib
// -------------
Promise.all([
    new Promise((resolve) => addEventListener("DOMContentLoaded", resolve)),
    WebAssembly.instantiateStreaming( fetch("../../../rust/sekistov/zig/lib.wasm"), { 
        env: {
            console_info,
            console_warn,
            console_err,
            console_err_allocate
        }
    })
]).then(main)

function main([_, r]) {
console.log('r: ', r)
    lib = r.instance.exports
    document.getElementById('upload-video-file-input').addEventListener('change', selectUploadVideoFile, false)
}

function selectUploadVideoFile(e) {
    for (const file of e.target.files) {
        console.log(file)
        var reader = new FileReader()
        reader.readAsArrayBuffer(file)
        const start_load = performance.now()
        reader.onload = function(e) {
            console.log('loaded ' + file.name, Math.round((performance.now() - start_load) * 1000))
            var contents = e.target.result
            const len = contents.byteLength 
            const timestamp = Math.round(file.lastModified / 1000)
            const textEncoder = new TextEncoder()
            const file_name_encoded = textEncoder.encode(file.name)
            console.log({file_name_encoded})
            const file_name_len = file_name_encoded.length
            // const arr = new Uint8Array(contents)
            const allocated_len = len + 4 + 1 + file_name_len;
            const ofs = lib.allocUint8(allocated_len);
            console.warn({allocated_len});
            (new Uint8Array(lib.memory.buffer, ofs, len)).set(new Uint8Array(contents));
            // console.log(lib)
            const compound = lib.getFileId(ofs, len)
            // console.log({compound})
            const file_id = strFromCompound(lib, compound)
// compound)
            // console_warn(file_id)

            fetch('/check/' + file_id).then(
                response => {
                    console.log(response)
                    if (200 == response.status) {
                        response.json().then(resp => { 
                            if (resp === null) {
                                (new Uint8Array(lib.memory.buffer, ofs + len + 4 + 1, file_name_encoded.length)).set(file_name_encoded);
                                lib.prepareBufToSend(ofs + len, timestamp)
                                const body = new Uint8Array(lib.memory.buffer, ofs, allocated_len)
                                // console.warn({allocated_len, byteLength: body.byteLength})
                                fetch('/upload', { 
                                    method: 'POST',
                                    headers: {
                                        'Content-Type': 'application/octet-stream', // https://stackoverflow.com/questions/6783921/which-mime-type-to-use-for-a-binary-file-thats-specific-to-my-program
                                    },
                                    body,
                                }).then(
                                    response => {
                                        console.log(response)
                                        // if (200 == response.status) {
                                        //     response.text().then(url => { 
                                        //         console.log({url})
                                        //         document.getElementById('video').setAttribute('src', '/' + url)
                                        //     })
                                        // }
                                    },
                                    err => {
                                        console.error(err)
                                    },
                                ) 
                            } else {
                                console.log({resp})
                            }
                            // document.getElementById('video').setAttribute('src', '/' + url)
                        })
                    }
                },
                err => {
                    console.error(err)
                },
            ) 
            // console.log(e)
        }
    }
}

function strFromCompound(lib, compound) {
    const {len, ofs} = fromCompound(compound);
    const str_buf = new Uint8Array(lib.memory.buffer, ofs, len);
    const str = new TextDecoder("utf8").decode(str_buf);
    lib.freeCompound(compound)
    return str
}
function fromCompound(compound) {
    const ofs = Number(BigInt(compound) & BigInt(0xFFFFFFFF));
    const len = Number(BigInt(compound) >> BigInt(34));
    return { ofs, len }
}



//////////////////////

</script>

<template>

<input type="file" id="upload-video-file-input" multiple/>

</template>

<style scoped>


.body { 
    margin: 0; 
    padding: 0; 
    position: absolute; 
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol";
    font-size: 16px;
    -webkit-tap-highlight-color: transparent;
    -webkit-text-size-adjust: 100%;
}

#video {
    display: block;
}

</style>
