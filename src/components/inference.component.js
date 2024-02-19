import { useEffect, useState } from "react";
import { sum_vec, test_gen_img, get_model, test_lazy_model, test_identify_bboxes } from "wasm-model";
import { WIDTH, HEIGHT } from "./carmera.component";

const IOU_THRESHOLD = 0.50;
const CONF_THRESHOLD = 0.50;


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
            test_identify_bboxes(sure_frame, CONF_THRESHOLD, IOU_THRESHOLD);
            const delta = performance.now() - now; 
            console.log("execution time: %s", delta)
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