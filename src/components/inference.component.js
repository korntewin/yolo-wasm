import { useEffect, useState } from "react";
import { sum_vec, test_gen_img, get_model, test_lazy_model, } from "wasm-model";

const WIDTH = 960;
const HEIGHT = 540;


const InferenceWebcam = ({ frame }) => {

    const [isLoaded, setIsLoaded] = useState(false);

    useEffect(() => {
        get_model().then(() => {
            console.log("Downloaded")
            setIsLoaded(true);
        })
    }, []);

    if (isLoaded) {
        const sure_frame = frame ? frame : [0];
        const sum_value = sum_vec(sure_frame);
        // test_gen_img(sure_frame, WIDTH, HEIGHT);
        const now = performance.now();
        test_lazy_model(sure_frame, WIDTH, HEIGHT);
        const delta = performance.now() - now;
        console.log("execution time: %.3f", delta)
        return (<div>{sum_value}, {sure_frame.length}</div>);
    } else {
        return <div>Loading...</div>;
    }
    // if (sure_frame.length > 1) {
    //     gen_img(sure_frame);
    // }


}


export default InferenceWebcam;