import { sum_vec } from "wasm-model";


const InferenceWebcam = ({ frame }) => {

    const sum_value = sum_vec(frame ? frame : [0]);

    return (<div>{sum_value}</div>);

}


export default InferenceWebcam;