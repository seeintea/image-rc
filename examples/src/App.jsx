import init, { resizeDataURL, compressDataURL } from "image-rc";
import { Fragment, useState } from "react";

const getImageList = async (url) => {
  const image = new Image();
  image.src = url;
  await new Promise((resolve) => {
    image.onload = () => resolve();
  });
  const { width, height } = image;
  let dataURL = "";
  {
    let canvas = document.createElement("canvas");
    canvas.width = width;
    canvas.height = height;
    const ctx = canvas.getContext("2d");
    ctx.drawImage(image, 0, 0);
    dataURL = canvas.toDataURL("image/png");
    canvas = null;
  }
  let start = performance.now();
  const double = resizeDataURL(dataURL, width * 2, height * 2);
  console.log("double: ", performance.now() - start);
  start = performance.now();
  const half = resizeDataURL(dataURL, width / 2, height / 2);
  console.log("half: ", performance.now() - start);
  start = performance.now();
  const compress = compressDataURL(dataURL, 0.6);
  console.log("compress: ", performance.now() - start);
  const doubleFile = new File([double], "double");
  const halfFile = new File([half], "half");
  const compressFile = new File([compress], "compress");
  const doubleUrl = URL.createObjectURL(doubleFile);
  const halfUrl = URL.createObjectURL(halfFile);
  const compressUrl = URL.createObjectURL(compressFile);
  return [doubleUrl, halfUrl, compressUrl];
};

const App = () => {
  const [images, setImages] = useState([]);
  const handleCreateImage = async () => {
    await init();
    console.log("init wasm!");
    const item = await getImageList("/images/demo-1.png");
    setImages([item]);
  };

  return (
    <div>
      <button onClick={handleCreateImage}>create images</button>
      {images.map((item) => (
        <Fragment key={JSON.stringify(item)}>
          {(item || []).map((image) => (
            <img key={image} src={image} alt="image" />
          ))}
        </Fragment>
      ))}
    </div>
  );
};

export default App;
