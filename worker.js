import init, { EncoderInput, convert_exposed } from "./pkg/image_convert.js";

let wasmReady = false;

async function initWasm() {
  await init();
  wasmReady = true;
  postMessage({ type: "ready" });
}

initWasm();

onmessage = async (event) => {
  if (!wasmReady) {
    postMessage({ type: "error", message: "WASM not ready yet" });
    return;
  }

  const { imageData, fileName, inputType, outputType, options: optionsRaw } = event.data;

  const options = new EncoderInput(
    optionsRaw.quality,
    optionsRaw.compression,
    optionsRaw.filter
  );

  try {
    const convertedImage = convert_exposed(imageData, inputType, outputType, options);

    postMessage({ type: "done", imageData: convertedImage, fileName, outputType }, [convertedImage.buffer]);
  } catch (err) {
    if (err instanceof WebAssembly.RuntimeError) {
      postMessage({ type: "error", message: "Unexpected WASM exception: " + err.message });
      console.error(err);
    } else {
      postMessage({ type: "error", message: err });
    }
  }
};
