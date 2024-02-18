import { sum_vec, gen_img } from "wasm-model";

const WIDTH = 1280;
const HEIGHT = 720;


const InferenceWebcam = ({ frame }) => {

    const sure_frame = frame ? frame : [0];
    const sum_value = sum_vec(sure_frame);
    gen_img(sure_frame, WIDTH, HEIGHT);
    // if (sure_frame.length > 1) {
    //     gen_img(sure_frame);
    // }

    return (<div>{sum_value}, {sure_frame.length}</div>);

}


export default InferenceWebcam;