import init, { favicon_check, favicon_generate } from './rust_favicon.js';

async function run() {
    await init();

    document.getElementById("GeneBtn").addEventListener("click", async (event) => {
        const files = document.getElementById("file_input").files;
        if(files.length === 0)
        {
            window.confirm("err:Fileをセットしてください");
            return;
        }
        let rdo_id ="";
        for(const ele of document.querySelectorAll("[name='rdo']"))
        {
            if(ele.checked)
            {
                rdo_id = ele.id;
                break;
            }
        }
        const file_blob = new Blob([files[0]], { type: files[0].type });
        await  blobToUint8Array(file_blob)
            .then(uint8Array => {
                favicon_generate(document.getElementById(rdo_id).value,uint8Array);
            })
            .catch(error => {
                console.error('Error converting blob:', error);
            });
    });

    document.getElementById("ChkBtn").addEventListener("click", async (event) => {
        const files = document.getElementById("file_checker").files;
        if(files.length === 0)
        {
            window.confirm("err:Fileをセットしてください");
            return;
        }
        const file_blob = new Blob([files[0]], { type: files[0].type });
        await blobToUint8Array(file_blob)
            .then(uint8Array => {
                document.getElementById("checker_output").innerText = favicon_check(uint8Array);
            })
            .catch(error => {
                console.error('Error converting blob:', error);
            });
    });

}
run();


async function blobToUint8Array(blob) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => {
            resolve(new Uint8Array(reader.result));
        };
        reader.onerror = reject;
        reader.readAsArrayBuffer(blob);
    });
}