import { useEffect, useState } from "react";
import { sum_vec, test_gen_img, get_model, test_lazy_model, } from "wasm-model";
import { WIDTH, HEIGHT } from "./carmera.component";


const InferenceWebcam = ({ frame }) => {

    const [isLoaded, setIsLoaded] = useState(false);

    useEffect(() => {
        get_model().then(() => {
            console.log("Downloaded")
            setIsLoaded(true);
        })
    }, []);

    if (isLoaded) {
        console.log("is loaded?: %s", isLoaded);

        if (frame) {
            const sure_frame = frame;
            const sum_value = sum_vec(sure_frame);
            const now = performance.now();
            // test_gen_img(sure_frame, WIDTH, HEIGHT);
            test_lazy_model(sure_frame, WIDTH, HEIGHT);
            const delta = performance.now() - now;
            console.log("execution time: %.3f", delta)
            return (<div>{sum_value}, {sure_frame.length}</div>);
        }

        return (<div>Frame is not available yet</div>)
    } else {
        return <div>Loading...</div>;
    }
    // if (sure_frame.length > 1) {
    //     gen_img(sure_frame);
    // }


}


export default InferenceWebcam;